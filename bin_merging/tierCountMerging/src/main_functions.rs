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

    let raw_data_aggregation_condition: usize = args[2].parse().expect("Provide a number");

    let num_tiers = args.len(); //= args[2].parse::<usize>().unwrap_or_default();

    let should_halt = Arc::new(AtomicBool::new(false));

    
    let aggregation_strategy: Arc<RwLock<dyn DataStrategy + Send + Sync>> = match data_strategy.as_str() {
        "count" => Arc::new(RwLock::new(CountRawData::new(raw_data_aggregation_condition))),
        "interval" => Arc::new(RwLock::new(IntervalRawData::new(raw_data_aggregation_condition))),
        _ => return Err("Invalid argument, please provide a valid data strategy".to_string()),
    };

    let (tiers,catch_all_policy)  = create_count_tiers(num_tiers, &args);

    // for tier in tiers {
    //     println!("Condition {}", tier.read().unwrap().condition);
    //     println!("Chunk size {}\n", tier.read().unwrap().chunk_size);
    // }
    // println!("{}", catch_all_policy);

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
            .map(|arg| arg.trim_end_matches(|c: char| !c.is_digit(10)))
            .and_then(|num_str| num_str.parse::<usize>().ok())
            .unwrap_or_default();

        tiers.push( Arc::new(RwLock::new(TierData::new( condition, 0))) );
        }
        let (condition, chunk_size, catch_all) =  create_count_catch_all(args, num_tiers-1);
        tiers.push( Arc::new(RwLock::new(TierData::new(condition, chunk_size))) );
        catch_all_policy = catch_all
    }

    (tiers, catch_all_policy)
}


pub fn create_count_catch_all(args: &[String], catch_all_index: usize) -> (usize, usize, bool) {
    let mut catch_all_policy = true;
    
    let condition = extract_number_before_c(&args[catch_all_index]);
    let chunk_size = extract_number_after_c(&args[catch_all_index]);

    if condition == 0 {
        catch_all_policy = false;
    }
    // Return the values as a tuple
    (condition, chunk_size, catch_all_policy)
}

fn extract_number_after_c(input: &str) -> usize {
    if let Some(index) = input.find('C') {
        let number_str = &input[index + 1..];
        number_str.parse::<usize>().unwrap_or_default()
    } else {
        0 // Default value if 'C' is not found
    }
}

fn extract_number_before_c(input: &str) -> usize {
    if let Some(index) = input.find('C') {
        let number_str = &input[..index];
        number_str.parse::<usize>().unwrap_or_default()
    } else {
        0 // Default value if 'C' is not found
    }
}