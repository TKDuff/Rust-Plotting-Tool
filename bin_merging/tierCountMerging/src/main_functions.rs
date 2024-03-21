use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::tier::{self, TierData};

use crate::count_data::CountRawData;
use crate::interval_data::IntervalRawData;


use crate::data_strategy::DataStrategy;
use std::env;

pub fn process_tier(current_tier: &Arc<RwLock<TierData>>, previous_tier: &Arc<RwLock<TierData>>, cut_length: usize) {
    let mut vec_len: usize;
    let current_tier_x_average;
    let current_tier_y_average;
    {
        let mut current_tier_lock = current_tier.write().unwrap();
        // print!("Before merge: ");
        // current_tier_lock.print_y_means_in_range(0, cut_length);
        let vec_slice = current_tier_lock.get_slices(cut_length);
        current_tier_x_average = current_tier_lock.merge_vector_bins(vec_slice.0);
        current_tier_y_average = current_tier_lock.merge_vector_bins(vec_slice.1);
        vec_len = current_tier_lock.x_stats.len();

        current_tier_lock.x_stats[0] = current_tier_x_average;
        //println!("The merged x stat is {}", current_tier_x_average.mean);
        current_tier_lock.x_stats.drain(1..vec_len-1);        

        current_tier_lock.y_stats[0] = current_tier_y_average;
        current_tier_lock.y_stats.drain(1..vec_len-1);

        // print!("After merge: ");
        // current_tier_lock.print_y_means_in_range(0, 2);
    }

    {
        let mut previous_tier_lock = previous_tier.write().unwrap();
        previous_tier_lock.x_stats.push(current_tier_x_average);
        previous_tier_lock.y_stats.push(current_tier_y_average);  
    }

}


pub fn setup_my_app() -> Result<(Arc<RwLock<dyn DataStrategy + Send + Sync>>, String, Vec<Arc<RwLock<TierData>>>, bool, Arc<AtomicBool>, usize), String> {
    let args: Vec<String> = env::args().collect();
    let mut data_strategy: String;
    let stdin_tier: usize; 

    //Cannot have more than 7 tiers, see error printed for rules
    if args.len()-2 > 7 {
        eprint!("Error\nMaximum of 7 tiers allowed, {} were specified\nCannot have more than 5 intermediate tiers\nCan only have 1 stdin tier and final (catch all) tier\n", args.len()-2);
        std::process::exit(1);
    }

    
    if args.len() == 1 { //Case when user does not any tiering*/
        println!("No arguments");
        data_strategy = "None".to_string();
    }  else if (args[1] == "--help") || (args[1] == "--h") {
        print_help();
        std::process::exit(1);
    } else {
        data_strategy = args[1].clone();
    }

        
    let num_tiers = args.len();
    let should_halt = Arc::new(AtomicBool::new(false));

    let (tiers, catch_all_policy, stdin_tier) = match data_strategy.as_str() {
        "count" => {
            let (tiers, catch_all_policy) = create_count_tiers(num_tiers, &args);
            //stdin_tier = args[2].parse().expect("Provide a number");
            stdin_tier = parse_positive_number("Stdin_data",&args[2], None);
            (tiers, catch_all_policy, stdin_tier)
        },
        "interval" => { 
            let (tiers, catch_all_policy) = create_inteval_tiers(num_tiers, &args);
            stdin_tier = convert_time_unit(&args[2]).unwrap_or_default();
            (tiers, catch_all_policy, stdin_tier)
        },
        "None" => {
            let (tiers, catch_all_policy) = create_dummy_tier();
            (tiers, catch_all_policy, 0)

        },
        _ => return Err("Provide a valid data strategy".to_string()),
    };

    //Trying to run the create tier methods on these branches is hard, too much to get working for now will leave it. Involes dynamic dispatch, screw that
    let aggregation_strategy: Arc<RwLock<dyn DataStrategy + Send + Sync>> = match data_strategy.as_str() {
        "count" => Arc::new(RwLock::new(CountRawData::new(stdin_tier))),
        "interval" => Arc::new(RwLock::new(IntervalRawData::new(stdin_tier))),
        "None" => Arc::new(RwLock::new(CountRawData::new(stdin_tier))),
        _ => return Err("Provide a valid data strategy".to_string()),
    };


    Ok((aggregation_strategy, data_strategy ,tiers, catch_all_policy ,should_halt, num_tiers))
}

//creates count based intermediate and catch all tiers based on the user command line arguments
fn create_count_tiers (num_tiers: usize, args: &[String]) -> (Vec<Arc<RwLock<TierData>>>, bool) {
    let mut tiers: Vec<Arc<RwLock<TierData>>> = Vec::new(); //vector that stores all the tiers
    let catch_all_policy: bool;   //flag if catch all policy 

    if num_tiers == 4 { //if user specify single tier, then only need to create the catch all as no intermediate tiers specified
        let (condition, chunk_size, catch_all) =  create_count_catch_all(args, num_tiers-1); //call method to return catch all tier values for tier constructor
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size, None))) ); //create catch all tier using values returned by method, push to 'tiers' vector
        catch_all_policy = catch_all
    } else {
        //loop to create all intermediate tiers, excluding catch all
        for i in 3..num_tiers-1 {

        let condition = parse_positive_number("Count", args.get(i).expect("Argument missing"), None); //ensure tier condition is number and > 0

        /*IMPORTANT
        The condition is incremented by 2 since every intermediate tier holds a reference to 2 points
        The last element of the tier behind it & first element of tier infront of it
        These 2 elements are never included in the merging process

        However condition checking if length of tier is equal to condition uses length of entire vector, including boundary elements
        Since they are excluded, could just decrement condition by 2, however going to increment the conditions by 2 at the start to make up for it
        So if tier condition is 1, the actual condition is 3. Need to exclude extra points
        */
        tiers.push( Arc::new(RwLock::new(TierData::new( condition+2, 0, None))) );
        }

        //create catch all tier, condition not incremented by 2 as catch all tier only contains its own bins
        let (condition, chunk_size, catch_all) =  create_count_catch_all(args, num_tiers-1);
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size, None))) );
        catch_all_policy = catch_all
    }

    (tiers, catch_all_policy)
}

//creates interval based intermediate and catch all tiers based on the user command line arguments
fn create_inteval_tiers (num_tiers: usize, args: &[String]) -> (Vec<Arc<RwLock<TierData>>>, bool) {
    let mut tiers: Vec<Arc<RwLock<TierData>>> = Vec::new();     //vector stores all the tiers created
    let mut previous_condition: usize = 0;
    let catch_all_policy: bool;

    if num_tiers == 4 {     //if user specify single tier, then only need to create the catch all as no intermediate tiers specified
        let (condition, chunk_size, catch_all) = create_interval_catch_all(args, num_tiers-1);  //call method to return catch all tier values for tier constructor
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size, Some(0)))) ); //create catch all tier using values returned by method, push to 'tiers' vector
        catch_all_policy = catch_all
    } else {
        //loop to create all intermediate tiers, excluding catch all
        for i in 3..num_tiers - 1 {
            let condition = convert_time_unit(&args[i]).unwrap_or_default();

            //Ensure the current condition is greater than the previous one. Does not make sense for the times to be out of order
            if condition <= previous_condition {
                eprintln!("Error: The intervals should be in ascending order\n");
            }
            previous_condition = condition;
            tiers.push( Arc::new(RwLock::new(TierData::new( condition, 0, None))) ); //create tier, only pass condition. No need for chunk size or to record time
        }
        let (condition, chunk_size, catch_all) = create_interval_catch_all(args, num_tiers-1); //create catch-all tier after all intermediate created

        if condition <= previous_condition && condition != 0 { //ensure catch all interval duration greater than all other tiers (previous one). Exclude 0 for '0C0' case
            eprintln!("Error: The intervals should be in ascending order\n");
        }
    
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size, Some(0)))) );
        catch_all_policy = catch_all
    }
    (tiers, catch_all_policy)
}

fn create_dummy_tier () -> (Vec<Arc<RwLock<TierData>>>, bool) {
    let tiers = vec![Arc::new(RwLock::new(TierData::new(0, 0, None)))]; 
    (tiers, true)
}


//Catch all policy either "0C0" or numbers N > 0 "N C N"
fn create_count_catch_all(args: &[String], catch_all_index: usize) -> (usize, usize, bool) {
    let mut catch_all_policy = true;
    let condition_str = extract_before_c(&args[catch_all_index]);
    let chunk_size = extract_after_c(&args[catch_all_index]).unwrap_or_default().parse::<usize>().unwrap_or_default(); //this looks terrible, calling unwrap twice, but it works
    let mut condition: usize = condition_str.parse::<usize>().unwrap_or_default();

    
    if condition == 0 && chunk_size == 0 { //if "0C0" no catch all policy, set it to false
        catch_all_policy = false;
    } else if chunk_size == 0 {
        eprintln!("Error: Final tier chunk size must be greater than 0 when condition is greater than 0, \"{}C0\"\nIf don't want to chunk final tier use \"0C0\" for tier argument\n", condition_str);
        std::process::exit(1);
    } else {
        condition = parse_positive_number("Final",&condition_str, Some(", condition can be 0 only if chunk size is 0, \"0C0\""));
    }


    // Return the values as a tuple
    (condition, chunk_size, catch_all_policy)
}

//Catch all policy either "0C0" or numbers N > 0 and Duration, D, needs to be included for number before the C, I.E "ND C N"
fn create_interval_catch_all(args: &[String], catch_all_index: usize) -> (usize, usize, bool) {
    let mut catch_all_policy = true;
    let time: usize;
    let condition_str = extract_before_c(&args[catch_all_index]); //get the interval number and duration, so 6M is 6 minutes or 360 seconds
    let chunk_size = extract_after_c(&args[catch_all_index]).unwrap_or_default().parse::<usize>().unwrap_or_default();

    if condition_str == "0" { 
        catch_all_policy = false;
        time = 0;
    } else if chunk_size == 0 {
        eprintln!("Error: Final tier chunk size must be greater than 0 when condition is greater than 0, \"{}C0\"\nIf don't want to chunk final tier use \"0C0\" for tier argument\n", condition_str);
        std::process::exit(1);
    } else {
        time = convert_time_unit(&condition_str).unwrap_or_default();   //conver the condition in to time in seconds
    }

    (time, chunk_size, catch_all_policy)

}


fn convert_time_unit(time_str: &str) -> Result<usize, String> {
    /*
    Method to ensure tier time argument is
    - number greater than 0
    - includes the duration symbol
    */
    let last_char = time_str.chars().last().unwrap_or_default();
    let number_part = &time_str[..time_str.len() - 1];

    let parsed_number = match number_part.parse::<usize>() {
        Ok(num) if num > 0 => num,
        _=> {
            eprintln!("Tier durations must be greater than 0 and contain duration symbol (S,M,H), you gave argument: {}\n", time_str);
            std::process::exit(1);
        },
    };

    match last_char {
        'S' => Ok(parsed_number),
        'M' => Ok(parsed_number * 60),
        'H' => Ok(parsed_number * 3600),
        _ => {
            eprint!("Tier durations must contain duration symbol (S,M,H), you gave argument: {}\n", time_str);
            std::process::exit(1);
        },
    }
}

fn parse_positive_number(tier:&str, arg: &str, final_tier: Option<&str>) -> usize {
    //If argument is a number and greater than 0, parse it to usize, return the value
    //Anything else throw an error
    //Used for count stdin and intermediate tiers
    match arg.parse::<usize>() {
        Ok(num) if num > 0 => num,
        _ => {
            if final_tier.is_some() {
                eprintln!("Error: {} tier condition must be positive numbers greater than 0{}\n", tier, final_tier.unwrap());
            } else {
                eprintln!("Error: {} tier condition must be postive numbers greater than 0\n", tier);
            }
            std::process::exit(1);
        }
    }
}

fn extract_before_c(input: &str) -> String {
    match input.find('C') {
        Some(index) => input[..index].to_string(),
        None => {
            eprintln!("Error: Final tier \"{}\" does not contain C character. Must specify final tier chunk policy with a C\n", input);
            std::process::exit(1);
        }
    }
}

fn extract_after_c(input: &str) -> Option<String> {
    match input.find('C') {
        Some(index) => Some(input[index + 1..].to_string()),
        None => None,
    }
}

fn print_help() {
    let help_message = r#"Usage: summarst [summarisation-policy] [stdin-data-condition] [tier-N-condition] ... [catch-all-tier]

summarisation-policy:
    Count: Aggregates elements into a bin whenever the specified number of elements is reached
    Interval: Aggregates elements obtained in a specified time interval into a single bin

stdin-data-condition:
    Count: Specify number of x, y points required in initial 'stdin-data' tier to be aggregated into a bin
    Example: '20' - every time 20 x, y points are plotted, aggregate them into a bin

    Interval: Specify time interval for which x, y values obtained during interval are aggregated into a bin
    Example: '2S' - when 2 seconds elapses, all the x, y points plotted in that interval are agregated

tier-N-condition:
    Tiers contain bins and up to 5 can be created (excluding the stdin-data tier and the catch all tier)
    When a tiers specified condition is met, its bins are 'merged' (combined) into a single bin which is pushed to the next tier
    The tier creation method depends on the chosen summarisation policy:

    Count: Specify the number of bins required for merging. 
    Example: '10 40' - when tier 1 contains 10 bins, they are merged and pushed to tier 2. When tier 2 contains 40 bins, they are merged and pushed to the next tier

    Interval: Specify time interval for merging. Ensure time intervals are in an ascending order
    Example: '30S 1M' - when 30 seconds elapses the bins in tier 1 are merged and pushed to tier 2. When 1 minute elapses the bins in tier 2 are merged and pushed to the next tier

catch-all-tier:
    Final tier, merges the bins in chunks of a specified size.
    Created similarly to other tiers, specifying when to merge bins and the chunk size
    Catch all tier policy MUST be included at end of tiering commands

    Count: Specify the number of bins required for merging and the chunk size. Both numbers separated by 'C'
    Example: '100C2' - every time 100 bins are obtained, merge the bins in chunks of size 2

    Interval: Specify time interval for merging and chunk size. C in middle to seperate the two numbers
    Example: '1HC10' - every hour merge the bins in chunks of size 10

    None: If don't want a catch all policy put '0C0' at the end
    Example: '0C0' - final tier won't merge in chunks, will grow over time

Interval Time Options:
    Three time durations to choose from below, smallest time interval possible is 1 second
    S - second
    M - minute
    H - hour

Examples:
    summarst - no arguments, wil plot x, y values as they are received from standard input

    Count:
    summarst count 5 10 0C0 - every 5 standard input points aggregated into a bin and pushed to next tier
                              every 10 bins obtained merged into a single bin and pushed to next final tier
                              no policy to merge bins, collect more over time

    summarst count 2 10 20C3 - every 2 standard input points aggregated into a bin and pushed to next tier
                               every 10 bins obtained merged into a single bin and pushed to next final tier
                               every 20 bins obtained, merge bins in chunks of size 3. Result in 6 merged bins and 1 remainder bin

    Interval:
    summarst interval 2S 0C0 - when 2 seconds elapse aggregate all x, y values obtained in time interval into a bin. Bin pushed to tier 1
                               Tier 1 does not have a catch all policy, will grow over time

    summarst interval 2S 10S 1M 10MC7 - merge points every 2 seconds and push to t1
                                       merge bins every 10 seconds, push merged bin to t2
                                       merge bins every 60 seconds, push merged bin to t3
                                       merge bins in chunks of 7 every 10 minutes

"#;

    println!("{}", help_message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Bin;

    #[test]
    fn test_process_tier() {
        let mut current_tier_data = Arc::new(RwLock::new(TierData::new(0, 0, None)));
        let mut previous_tier_data = Arc::new(RwLock::new(TierData::new(0, 0, None)));
        let x_dummy_bins = Bin::create_uniform_bins(5.0, 10); // for example: 5 bins with all values set to 5.0
        current_tier_data.write().unwrap().x_stats = x_dummy_bins.clone();
        current_tier_data.write().unwrap().y_stats = x_dummy_bins;

        let y_dummy_bins = Bin::create_uniform_bins(5.0, 10);
        previous_tier_data.write().unwrap().x_stats = y_dummy_bins.clone();
        previous_tier_data.write().unwrap().y_stats = y_dummy_bins;

        assert_eq!(current_tier_data.read().unwrap().x_stats.len(), 10);

        process_tier(&current_tier_data, &previous_tier_data, 10);

        //Verify the length of current_tier's x_stats and y_stats after drain
        assert_eq!(current_tier_data.read().unwrap().x_stats.len(), 2); // Assuming it gets reduced to 2
        assert_eq!(current_tier_data.read().unwrap().y_stats.len(), 2); // Assuming it gets reduced to 2


        //first element x_stats, y_stats is averaged bin
        assert_eq!(current_tier_data.read().unwrap().x_stats[0].mean, 5.0);
        assert_eq!(current_tier_data.read().unwrap().y_stats[0].mean, 5.0);

        //last element previous_tier x_stats, y_stats is average from current_tier (pushed to previour tier)
        assert_eq!(previous_tier_data.read().unwrap().x_stats[5].mean, 5.0);
        assert_eq!(previous_tier_data.read().unwrap().y_stats[5].mean, 5.0);

        /*
        // Setup test data
        let current_tier_data = TierData::new(); // Populate this with test data
        let previous_tier_data = TierData::new(); // Populate this if necessary
        let current_tier = Arc::new(RwLock::new(current_tier_data));
        let previous_tier = Arc::new(RwLock::new(previous_tier_data));
        let cut_length = 5; // Example value

        // Call the function
        process_tier(&current_tier, &previous_tier, cut_length);

        // Verify the results
        {
            let current_tier_lock = current_tier.read().unwrap();
            let previous_tier_lock = previous_tier.read().unwrap();

            // Assertions go here
            // Example:
            // assert_eq!(current_tier_lock.x_stats.len(), expected_length);
            // assert_eq!(previous_tier_lock.y_stats.last(), Some(&current_tier_y_average));
            // ...
        }*/
    }


    
}