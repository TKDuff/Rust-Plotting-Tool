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
        //print!("Before merge: ");
        //current_tier_lock.print_y_means_in_range(0, cut_length);
        let vec_slice = current_tier_lock.get_slices(cut_length);
        current_tier_x_average = current_tier_lock.merge_vector_bins(vec_slice.0);
        current_tier_y_average = current_tier_lock.merge_vector_bins(vec_slice.1);
        vec_len = current_tier_lock.x_stats.len();

        current_tier_lock.x_stats[0] = current_tier_x_average;
        //println!("The merged x stat is {}", current_tier_x_average.mean);
        current_tier_lock.x_stats.drain(1..vec_len-1);        

        current_tier_lock.y_stats[0] = current_tier_y_average;
        current_tier_lock.y_stats.drain(1..vec_len-1);

        //print!("After merge: ");
        //current_tier_lock.print_y_means_in_range(0, 2);
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
    let raw_data_aggregation_condition: usize; 


    /*Case when user does not any tiering*/
    if args.len() == 1 {
        println!("No arguments");
        data_strategy = "None".to_string();
    }  else if (args[1] == "--help") || (args[1] == "--h") {
        print_help();
        std::process::exit(0);
    } else {
        data_strategy = args[1].clone();
    }

        
    let num_tiers = args.len();
    let should_halt = Arc::new(AtomicBool::new(false));

    let (tiers, catch_all_policy, raw_data_aggregation_condition) = match data_strategy.as_str() {
        "count" => {
            let (tiers, catch_all_policy) = create_count_tiers(num_tiers, &args);
            raw_data_aggregation_condition = args[2].parse().expect("Provide a number");
            (tiers, catch_all_policy, raw_data_aggregation_condition)
        },
        "interval" => { 
            let (tiers, catch_all_policy) = create_inteval_tiers(num_tiers, &args);
            raw_data_aggregation_condition = convert_time_unit(&args[2]).unwrap_or_default();
            (tiers, catch_all_policy, raw_data_aggregation_condition)
        },
        "None" => {
            let (tiers, catch_all_policy) = create_dummy_tier();
            (tiers, catch_all_policy, 0)

        },
        _ => return Err("Provide a valid data strategy".to_string()),
    };

    //Trying to run the create tier methods on these branches is hard, too much to get working for now will leave it. Involes dynamic dispatch, screw that
    let aggregation_strategy: Arc<RwLock<dyn DataStrategy + Send + Sync>> = match data_strategy.as_str() {
        "count" => Arc::new(RwLock::new(CountRawData::new(raw_data_aggregation_condition))),
        "interval" => Arc::new(RwLock::new(IntervalRawData::new(raw_data_aggregation_condition))),
        "None" => Arc::new(RwLock::new(CountRawData::new(raw_data_aggregation_condition))),
        _ => return Err("Provide a valid data strategy".to_string()),
    };


    Ok((aggregation_strategy, data_strategy ,tiers, catch_all_policy ,should_halt, num_tiers))
}

fn create_count_tiers (num_tiers: usize, args: &[String]) -> (Vec<Arc<RwLock<TierData>>>, bool) {
    let mut tiers = Vec::new();
    let catch_all_policy;

    if num_tiers == 4 { //if there is only a sinlge argument, beside the raw data argument
        let (condition, chunk_size, catch_all) =  create_count_catch_all(args, num_tiers-1);
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size, None))) );
        catch_all_policy = catch_all
    } else {
        for i in 3..num_tiers-1 {
            let condition = args.get(i)
            .and_then(|arg| arg.parse::<usize>().ok())
            .unwrap_or_default();

        tiers.push( Arc::new(RwLock::new(TierData::new( condition, 0, None))) );
        }
        let (condition, chunk_size, catch_all) =  create_count_catch_all(args, num_tiers-1);
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size, None))) );
        catch_all_policy = catch_all
    }

    (tiers, catch_all_policy)
}

fn create_inteval_tiers (num_tiers: usize, args: &[String]) -> (Vec<Arc<RwLock<TierData>>>, bool) {
    let mut tiers = Vec::new();
    let mut previous_condition = 0;
    let catch_all_policy: bool;

    if num_tiers == 4 {
        let (condition, chunk_size, catch_all) = create_interval_catch_all(args, num_tiers-1);
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size, Some(0)))) );
        catch_all_policy = catch_all
    } else {
        for i in 3..num_tiers - 1 {
            let condition = convert_time_unit(&args[i]).unwrap_or_default();

            //Ensure the current condition is greater than the previous one. Does not make sense for the times to be out of orderW``
            if condition <= previous_condition {
                panic!("The intervals should be in ascending order");
            }
            previous_condition = condition;
            tiers.push( Arc::new(RwLock::new(TierData::new( condition, 0, None))) );
        }
        let (condition, chunk_size, catch_all) = create_interval_catch_all(args, num_tiers-1);

        if condition <= previous_condition && condition != 0 {
            panic!("The intervals should be in ascending order");
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

fn create_count_catch_all(args: &[String], catch_all_index: usize) -> (usize, usize, bool) {
    let mut catch_all_policy = true;
    
    let condition_str = extract_before_c(&args[catch_all_index]).unwrap_or_default();
    let chunk_size = extract_after_c(&args[catch_all_index]).unwrap_or_default().parse::<usize>().unwrap_or_default(); //this looks terrible, calling unwrap twice, but it works

    let condition = condition_str.parse::<usize>().unwrap_or_default();

    
    if condition == 0 {
        catch_all_policy = false;
    } else if chunk_size == 0 {
        eprintln!("Final tier chunk size cannot be 0");
        std::process::exit(1); // Exits the program
    } else if chunk_size == 1 {
        println!("Warning: final tier chunk size is 1, instead make it \"0C\""); //Put this on egui if don't want to exit
        std::process::exit(1);
    }

    // Return the values as a tuple
    (condition, chunk_size, catch_all_policy)
}

fn create_interval_catch_all(args: &[String], catch_all_index: usize) -> (usize, usize, bool) {
    let mut catch_all_policy = true;

    let condition_str = extract_before_c(&args[catch_all_index]).unwrap_or_default(); //get the interval number and duration, so 6M is 6 minutes or 360 seconds
    let chunk_size = extract_after_c(&args[catch_all_index]).unwrap_or_default().parse::<usize>().unwrap_or_default();
    let time = convert_time_unit(&condition_str).unwrap_or_default();   //conver the condition in to time in seconds

    if time == 0 {
        catch_all_policy = false;
    } else if chunk_size == 0 {
        eprintln!("Final tier chunk size cannot be 0");
        std::process::exit(1); // Exits the program
    } else if chunk_size == 1 {
        println!("Warning: final tier chunk size is 1, instead make it \"0C\""); //Put this on egui if don't want to exit
        std::process::exit(1);
    }

    (time, chunk_size, catch_all_policy)

}

fn convert_time_unit(time_str: &str) -> Result<usize, String> {
    let last_char = time_str.chars().last().unwrap_or_default();
    let number_part = &time_str[..time_str.len() - 1];
    
    match last_char {
        'S' | 's' => number_part.parse().map_err(|_| "Invalid number".to_string()),
        'M' | 'm' => number_part.parse::<usize>()
                     .map(|minutes| minutes * 60)
                     .map_err(|_| "Invalid number".to_string()),
        'H' | 'h' => number_part.parse::<usize>()
                     .map(|hours| hours * 3600)
                     .map_err(|_| "Invalid number".to_string()),
        _ => Err("Invalid time unit".to_string()),
    }
}

fn extract_after_c(input: &str) -> Option<String> {
    input.find('C').map(|index| input[index + 1..].to_string())
}

fn extract_before_c(input: &str) -> Option<String> {
    input.find('C').map(|index| input[..index].to_string())
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