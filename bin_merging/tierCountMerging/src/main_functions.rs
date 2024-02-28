use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::tier::TierData;

use crate::count_data::CountRawData;
use crate::interval_data::IntervalRawData;


use crate::data_strategy::DataStrategy;
use std::{env, num};

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


pub fn setup_my_app() /*-> Result<(Arc<RwLock<dyn DataStrategy + Send + Sync>>, String, Vec<Arc<RwLock<TierData>>>, bool, Arc<AtomicBool>, usize), String> */ {
    let args: Vec<String> = env::args().collect();
    let data_strategy = args[1].clone();

    let raw_data_aggregation_condition: usize;
    let num_tiers = args.len(); 
    let should_halt = Arc::new(AtomicBool::new(false));

    let aggregation_strategy: Arc<RwLock<dyn DataStrategy + Send + Sync>>;
    let tiers: Vec<Arc<RwLock<TierData>>>;
    let catch_all_policy: bool;


    /*
    match data_strategy.as_str() {
        "count" => {
            let raw_data_aggregation_condition: usize = args[2].parse().expect("Provide a number");
            aggregation_strategy = Arc::new(RwLock::new(CountRawData::new(raw_data_aggregation_condition)));
            let result = create_count_tiers(num_tiers, &args);
            tiers = result.0;
            catch_all_policy = result.1;
        },
        "interval" => {
            aggregation_strategy = Arc::new(RwLock::new(IntervalRawData::new(raw_data_aggregation_condition)));
            let result = interval_interval_parser(num_tiers, &args);
            // tiers = result.0;
            // catch_all_policy = result.1;
        },
        _ => return Err("Invalid argument, please provide a valid data strategy".to_string()),
    };*/


    create_interval_tiers(num_tiers, &args);

    //Ok((aggregation_strategy, data_strategy ,tiers, catch_all_policy ,should_halt, num_tiers))
}


fn create_count_tiers(num_tiers: usize, args: &[String]) -> (Vec<Arc<RwLock<TierData>>>, bool) {

    let mut tiers = Vec::new();
    let mut catch_all_policy = true;
    //println!("{:?}", args);

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

fn create_interval_tiers(num_tiers: usize, args: &[String]) {
    let mut tiers = Vec::new();
    let mut catch_all_policy = true;

    for i in 3..num_tiers {
        //let condition_str = args.get(i);
        let condition_str = args.get(i).map(|s| s.as_str()).unwrap_or("");
        let condition = convert_time_unit(condition_str);
        println!("{}", condition);
        let tier = Arc::new(RwLock::new(TierData::new( condition )));
        tiers.push(tier);
    }

    if tiers[num_tiers-4].read().unwrap().condition == 0 {
        catch_all_policy = false;
    }


}

fn convert_time_unit(time_str: &str) -> usize {
    if let Some(unit) = time_str.chars().last() {
        let number = time_str.trim_end_matches(unit).parse::<usize>().unwrap_or_default();
        match unit {
            'S' | 's' => number,          // Seconds
            'M' | 'm' => number * 60,     // Minutes
            'H' | 'h' => number * 3600,   // Hours
            _ => 0,                       // Unrecognized unit
        }
    } else {
        0 // No unit provided
    }
}