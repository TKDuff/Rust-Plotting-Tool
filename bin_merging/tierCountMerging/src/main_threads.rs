
use std::{num, thread, usize};
use crossbeam::channel::Receiver;
use crate::data_strategy::DataStrategy;
use crate::tier::TierData;
use std::sync::{Arc, RwLock};
use tokio::time::Duration;
use crate::main_functions::process_tier;
use crate::bin::Bin;

pub fn create_raw_data_to_initial_tier(hd_receiver: Receiver<usize>, raw_data_accessor: Arc<RwLock<dyn DataStrategy + Send + Sync>>, initial_tier_accessor: Arc<RwLock<TierData>> )   {
    thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut objective_length = 0;
        let mut aggregated_raw_data ; 
        for message in hd_receiver {

            {
                println!("{}", message);
                let mut aggregate_thread_raw_data_accessor_lock = raw_data_accessor.write().unwrap();
                chunk = aggregate_thread_raw_data_accessor_lock.get_chunk(message);
                aggregated_raw_data = aggregate_thread_raw_data_accessor_lock.append_chunk_aggregate_statistics(chunk);
                aggregate_thread_raw_data_accessor_lock.remove_chunk(message);
            }

            {
                let mut initial_tier_lock = initial_tier_accessor.write().unwrap();
                let length = initial_tier_lock.x_stats.len() -1 ;
                initial_tier_lock.x_stats[length] = aggregated_raw_data.2;
                initial_tier_lock.y_stats[length] = aggregated_raw_data.3;

                initial_tier_lock.x_stats.push(aggregated_raw_data.0);
                initial_tier_lock.y_stats.push(aggregated_raw_data.1);
            }
            
        }   
    });
}


pub fn create_raw_data_to_initial_tier_edge(hd_receiver: Receiver<usize>, raw_data_accessor: Arc<RwLock<dyn DataStrategy + Send + Sync>>, initial_tier_accessor: Arc<RwLock<TierData>>) {
    thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut new_bin_x: Vec<Bin>;
        let mut new_bin_y: Vec<Bin>;
        for message in hd_receiver {
            {
                let mut aggregate_thread_raw_data_accessor_lock = raw_data_accessor.write().unwrap();
                chunk = aggregate_thread_raw_data_accessor_lock.get_chunk(message);


                new_bin_x = chunk.iter()
                .map(|&[x_mean, _]| Bin {
                    mean: x_mean,
                    sum: x_mean,
                    min: x_mean,
                    max: x_mean,
                    count: 1,
                }).collect();

                new_bin_y = chunk.iter()
                .map(|&[_, y_mean]| Bin {
                    mean: y_mean,
                    sum: y_mean,
                    min: y_mean,
                    max: y_mean,
                    count: 1,
                }).collect();
            }

            {
                let mut initial_tier_accessor_lock = initial_tier_accessor.write().unwrap();
                initial_tier_accessor_lock.x_stats.extend(new_bin_x);
                initial_tier_accessor_lock.y_stats.extend(new_bin_y);

                let length = initial_tier_accessor_lock.x_stats.len();
            
                initial_tier_accessor_lock.merge_final_tier_vector_bins(2, length-1,true);
                initial_tier_accessor_lock.merge_final_tier_vector_bins(2, length-1,false);
                
            }

            {
                let mut aggregate_thread_raw_data_accessor_lock = raw_data_accessor.write().unwrap();
                aggregate_thread_raw_data_accessor_lock.remove_chunk(message);
            }
        }
    });
}

pub fn create_tier_check_cut_loop(tier_vector :Vec<Arc<RwLock<TierData>>>, catch_all_tier: Arc<RwLock<TierData>>, num_tiers: usize) {
    thread::spawn(move || { 
        let mut merged_CA_last_x_element;
        let mut merged_CA_last_y_element;
        let CA_condition = catch_all_tier.read().unwrap().condition;

        
        loop {
            //will break when only one tier, however this is an edge case, the Catch All only edge case
            for tier in 0..=(num_tiers-2) {  //only testing on first tier, initial tier, for now 
                if tier_vector[tier].read().unwrap().x_stats.len() == tier_vector[tier].read().unwrap().condition {
                    //println!("\nTier {}", tier);
                    print!("{:?}", tier_vector[tier].read().unwrap().print_x_means("Before"));
                    process_tier(&tier_vector[tier], &tier_vector[tier+1], 7);
                    //print!("{:?}\n", tier_vector[tier].read().unwrap().print_x_means("After"));
                }
                
            } 
            thread::sleep(Duration::from_millis(1));

            let mut catch_all_tier_write_lock = catch_all_tier.write().unwrap();

            if catch_all_tier_write_lock.x_stats.len() == CA_condition {
                
                merged_CA_last_x_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(3,CA_condition, true);
                merged_CA_last_y_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(3,CA_condition, false);
                println!("Got the point {:?}", merged_CA_last_x_element);

                let mut tier_vector_write_lock = tier_vector[num_tiers-2].write().unwrap();

                println!("The first elem of t2 was {:?}", tier_vector_write_lock.x_stats[0]);
                tier_vector_write_lock.x_stats[0] = merged_CA_last_x_element;
                tier_vector_write_lock.y_stats[0] = merged_CA_last_y_element;
                println!("Now the first elem of t2 is {:?}", tier_vector_write_lock.x_stats[0]);

            }
        }
    });
}