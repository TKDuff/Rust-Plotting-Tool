use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::tier::TierData;

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


pub fn setup_my_app() -> Result<(Arc<RwLock<dyn DataStrategy + Send + Sync>>, Vec<Arc<RwLock<TierData>>>, bool, Arc<AtomicBool>, usize), String> {
    let args: Vec<String> = env::args().collect();
    let data_strategy = args[1].as_str();

    let raw_data_aggregation_condition: usize = args[2].parse().expect("Provide a number");

    let num_tiers = args.len(); //= args[2].parse::<usize>().unwrap_or_default();

    let should_halt = Arc::new(AtomicBool::new(false));

    let strategy: Arc<RwLock<dyn DataStrategy + Send + Sync>> = match data_strategy {
        "count" => Arc::new(RwLock::new(CountRawData::new(raw_data_aggregation_condition))),
        "interval" => Arc::new(RwLock::new(IntervalRawData::new(raw_data_aggregation_condition))),
        _ => return Err("Invalid argument, please provide a valid data strategy".to_string()),
    };

    let (tiers,catch_all_policy)  = create_tiers(num_tiers, &args);

    Ok((strategy, tiers, catch_all_policy ,should_halt, num_tiers))
}


fn create_tiers(num_tiers: usize, args: &[String]) -> (Vec<Arc<RwLock<TierData>>>, bool) {

    let mut tiers = Vec::new();
    let mut catch_all_policy = true;
    println!("{:?}", args);

    for i in 3..num_tiers {
        let condition = args.get(i)
            .map(|arg| arg.trim_end_matches(|c: char| !c.is_digit(10)))
            .and_then(|num_str| num_str.parse::<usize>().ok())
            .unwrap_or_default();
        
        let tier = Arc::new(RwLock::new(TierData::new( condition )));
        
        tiers.push(tier);
    }

    if tiers[num_tiers-4].read().unwrap().condition == 0 {
        catch_all_policy = false;
    }    
    (tiers, catch_all_policy)
}