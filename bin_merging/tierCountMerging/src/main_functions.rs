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
        let vec_slice = current_tier_lock.get_slices(cut_length);
        current_tier_x_average = current_tier_lock.merge_vector_bins(vec_slice.0);
        current_tier_y_average = current_tier_lock.merge_vector_bins(vec_slice.1);
        vec_len = current_tier_lock.x_stats.len();

        current_tier_lock.x_stats[0] = current_tier_x_average;
        current_tier_lock.x_stats.drain(1..vec_len-1);

        current_tier_lock.y_stats[0] = current_tier_y_average;
        current_tier_lock.y_stats.drain(1..vec_len-1);
    }

    {
        let mut previous_tier_lock = previous_tier.write().unwrap();
        previous_tier_lock.x_stats.push(current_tier_x_average);
        previous_tier_lock.y_stats.push(current_tier_y_average);  
    }

}


pub fn setup_my_app() -> Result<(Arc<RwLock<dyn DataStrategy + Send + Sync>>, String, Vec<Arc<RwLock<TierData>>>, bool, Arc<AtomicBool>, usize), String> {
    let args: Vec<String> = env::args().collect();
    let data_strategy = args[1].clone();

    let raw_data_aggregation_condition: usize; 

    let num_tiers = args.len(); //= args[2].parse::<usize>().unwrap_or_default();

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
        _ => return Err("Provide a valid data strategy".to_string()),
    };

    //Trying to run the create tier methods on these branches is hard, too much to get working for now will leave it. Involes dynamic dispatch, screw that
    let aggregation_strategy: Arc<RwLock<dyn DataStrategy + Send + Sync>> = match data_strategy.as_str() {
        "count" => Arc::new(RwLock::new(CountRawData::new(raw_data_aggregation_condition))),
        "interval" => Arc::new(RwLock::new(IntervalRawData::new(raw_data_aggregation_condition))),
        _ => return Err("Provide a valid data strategy".to_string()),
    };


    /*
    for tier in tiers {
        println!("{}", tier.read().unwrap().condition);
        println!("{}", tier.read().unwrap().chunk_size);
        println!("");
    }
    println!("{}", catch_all_policy);*/

    Ok((aggregation_strategy, data_strategy ,tiers, catch_all_policy ,should_halt, num_tiers))
}

fn create_count_tiers (num_tiers: usize, args: &[String]) -> (Vec<Arc<RwLock<TierData>>>, bool) {
    let mut tiers = Vec::new();
    let catch_all_policy;

    if num_tiers == 4 { //if there is only a sinlge argument, beside the raw data argument
        let (condition, chunk_size, catch_all) =  create_count_catch_all(args, num_tiers-1);
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size))) );
        catch_all_policy = catch_all
    } else {
        for i in 3..num_tiers-1 {
            let condition = args.get(i)
            .and_then(|arg| arg.parse::<usize>().ok())
            .unwrap_or_default();

        tiers.push( Arc::new(RwLock::new(TierData::new( condition, 0))) );
        }
        let (condition, chunk_size, catch_all) =  create_count_catch_all(args, num_tiers-1);
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size))) );
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
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size))) );
        catch_all_policy = catch_all
    } else {
        for i in 3..num_tiers - 1 {
            let condition = convert_time_unit(&args[i]).unwrap_or_default();

            //Ensure the current condition is greater than the previous one. Does not make sense for the times to be out of orderW``
            if condition <= previous_condition {
                panic!("The intervals should be in ascending order");
            }
            previous_condition = condition;
            tiers.push( Arc::new(RwLock::new(TierData::new( condition, 0))) );
        }
        let (condition, chunk_size, catch_all) = create_interval_catch_all(args, num_tiers-1);

        if condition <= previous_condition && condition != 0 {
            panic!("The intervals should be in ascending order");
        }
    
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size))) );
        catch_all_policy = catch_all
    }
    (tiers, catch_all_policy)
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