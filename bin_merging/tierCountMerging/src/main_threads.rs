
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, usize};
use crossbeam::channel::{Receiver, Sender};
use crate::data_strategy::DataStrategy;
use crate::tier::TierData;
use std::sync::{Arc, RwLock};
use tokio::time::{self,Duration, Instant};
use crate::main_functions::process_tier;
use crate::bin::Bin;
use tokio::runtime::Runtime;
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub fn create_count_stdin_read(rt: &Runtime, should_halt_clone: Arc<AtomicBool>, raw_data_thread: Arc<RwLock<dyn DataStrategy + Send + Sync>>, rd_sender: Sender<usize>) {
    rt.spawn(async move {
        println!("create_count_stdin_read");
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        raw_data_thread.write().unwrap().append_str(line);
                        if let Some(cut_index) = raw_data_thread.write().unwrap().check_cut() {
                            rd_sender.send(cut_index).unwrap();
                        }
                    } else {
                        break;
                    }
                }, 
            }

        }
    });
}

pub fn create_interval_stdin_read(rt: &Runtime, should_halt_clone: Arc<AtomicBool>, raw_data_thread: Arc<RwLock<dyn DataStrategy + Send + Sync>>, rd_sender: Sender<usize>) {
    let raw_data_interval_condition = raw_data_thread.read().unwrap().get_condition();

    let initial_delay = Duration::from_secs(raw_data_interval_condition as u64);
    let interval_duration = Duration::from_secs(raw_data_interval_condition as u64); 

    rt.spawn(async move {

        let mut interval = time::interval_at(Instant::now() + initial_delay, interval_duration);
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            if should_halt_clone.load(Ordering::SeqCst) {
                break; // Exit the loop if the atomic bool is true
            }
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        raw_data_thread.write().unwrap().append_str(line);
                    } else {
                        break;
                    }
                }, 
                _ = interval.tick() => {
                    let length = raw_data_thread.read().unwrap().get_length();
                    rd_sender.send(length).unwrap();

                }
            }
        }
    });
}

pub fn crate_none_stdin_read(rt: &Runtime, should_halt_clone: Arc<AtomicBool>, raw_data_thread: Arc<RwLock<dyn DataStrategy + Send + Sync>>) {
    rt.spawn(async move {
        println!("create_count_stdin_read");
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            if should_halt_clone.load(Ordering::SeqCst) {
                break; // Exit the loop if the atomic bool is true
            }
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        raw_data_thread.write().unwrap().append_str(line);
                    } else {
                        break;
                    }
                }, 
            }
        }
    });
}

pub fn create_raw_data_to_initial_tier(hd_receiver: Receiver<usize>, raw_data_accessor: Arc<RwLock<dyn DataStrategy + Send + Sync>>, initial_tier_accessor: Arc<RwLock<TierData>> )   {
    thread::spawn(move || {
        println!("create_raw_data_to_initial_tier");
        let mut chunk: Vec<[f64;2]>;
        let mut aggregated_raw_data ; 
        for message in hd_receiver {
            {
                let mut aggregate_thread_raw_data_accessor_lock = raw_data_accessor.write().unwrap();
                chunk = aggregate_thread_raw_data_accessor_lock.get_chunk(message);

                //return last element of the R.D for x,y and the aggregated R.D excluding the last point
                aggregated_raw_data = aggregate_thread_raw_data_accessor_lock.append_chunk_aggregate_statistics(chunk);
                aggregate_thread_raw_data_accessor_lock.remove_chunk(message);
            }

            {
                let mut initial_tier_lock = initial_tier_accessor.write().unwrap();
                let length = initial_tier_lock.x_stats.len() -1 ;
                initial_tier_lock.x_stats[length] = aggregated_raw_data.2;
                initial_tier_lock.y_stats[length] = aggregated_raw_data.3;

                //println!("R.D last element {}", aggregated_raw_data.0.mean);
                //println!("R.D last element {}", aggregated_raw_data.1.mean);
                initial_tier_lock.x_stats.push(aggregated_raw_data.0);
                initial_tier_lock.y_stats.push(aggregated_raw_data.1);
            }
            
        }   
    });
}


pub fn interval_rd_to_ca_edge(initial_tier_accessor: Arc<RwLock<TierData>>) {
    println!("interval_rd_to_ca_edge");

    /*
    Edge case, just like count_rd_to_ca_edge
    If user only has stdin_data tier & catch-all-tier with no intermediate tiers this thread is spawned
    The catch-all-tier becomes the initial tier (tier 1)
    Since t1 contains a reference to the first element of the stdin_data tier, in this case the catch-all-tier contains a reference to the first point of stdin_data
    This catch-all-tier does not just contain bins, the final element is an un-aggregated x,y point from stdin_data
    Different to non-edge case catch all tier which only contains bins in its tier, the last element of it is the final bin of the tier. No reference to next tier.

    This difference means that the length includes an additional non-bin. 
    As not a bin, final element of tier vector excluded. Thus the length is decremented by 1.
    
     */
    thread::spawn(move || {
        let mut seconds_passed: usize = 1;
        let mut catch_all_length: usize;
        thread::sleep(Duration::from_secs(1));
        let ca_condition = initial_tier_accessor.read().unwrap().condition;
        let ca_chunk_size: usize = initial_tier_accessor.read().unwrap().chunk_size;

        loop {
            //if users selects '0C0' for catch-all-only edge case, no point in trying to modulo seconds passed by 0 (ca_condition in this case). This loop is polling nothing, should be removed
            if ca_condition != 0 &&  seconds_passed % ca_condition == 0 {
                println!("merge sec {}", seconds_passed); 
                {
                let mut catch_all_tier_write_lock = initial_tier_accessor.write().unwrap();            
                catch_all_length = catch_all_tier_write_lock.x_stats.len()-1;
                catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, true);
                catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, false);
                println!("After meging the tier it becomes");
                for bin in &catch_all_tier_write_lock.y_stats {
                    print!("{}, ", bin.mean);
                }
                println!("\n"); 
                }
            }
        initial_tier_accessor.write().unwrap().time_passed = Some(seconds_passed);
        seconds_passed += 1;
        //println!("Seconds passed {}", seconds_passed);
        thread::sleep(Duration::from_secs(1));
        }
    });
}


pub fn count_rd_to_ca_edge(initial_tier_accessor: Arc<RwLock<TierData>>) {
    /*
    Edge case
    If user only has stdin_data tier & catch-all-tier with no intermediate tiers this thread is spawned
    The catch-all-tier becomes the initial tier (tier 1)
    Since t1 contains a reference to the first element of the stdin_data tier, in this case the catch-all-tier contains a reference to the first point of stdin_data
    This catch-all-tier does not just contain bins, the final element is an un-aggregated x,y point from stdin_data
    Different to non-edge case catch all tier which only contains bins in its tier, the last element of it is the final bin of the tier. No reference to next tier.

    This difference means that the length includes an additional non-bin. 
    As not a bin, final element of tier vector excluded. Thus the length is decremented by 1. 
    This is also reflected in GUI, around line 270. The condition checks for this edge case. 
    If edge case met and only stdin_tier and catch-all-tier no need to iterate over all tiers, just display length of both
    The length of the catac-all-tier on the GUI is decremented by 1 also, for the same reason. Since last point not included in chunking, should not be considered part of length.  
    */
    println!("count_rd_to_ca_edge");
    thread::spawn(move || {
        let mut catch_all_length: usize;
        thread::sleep(Duration::from_secs(1));
        let ca_condition = initial_tier_accessor.read().unwrap().condition;
        let ca_chunk_size: usize = initial_tier_accessor.read().unwrap().chunk_size;

        loop {
            catch_all_length = initial_tier_accessor.read().unwrap().x_stats.len()-1;
            if catch_all_length == ca_condition {
                println!("MERGE TICK");
                {
                let mut catch_all_tier_write_lock = initial_tier_accessor.write().unwrap();            
                catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, true);
                catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, false);

                println!("After meging the tier it becomes");
                for bin in &catch_all_tier_write_lock.y_stats {
                    print!("{}, ", bin.mean);
                }
                println!("\n");
                }
            }
        }
    });
}



pub fn interval_check_cut_ca(tier_vector :Vec<Arc<RwLock<TierData>>>, catch_all_tier: Arc<RwLock<TierData>>, num_tiers: usize) {
    println!("interval_check_cut_ca");
    thread::spawn(move || {        
        let mut merged_CA_last_x_element;
        let mut merged_CA_last_y_element;
        let CA_condition = catch_all_tier.read().unwrap().condition;  
        let ca_chunk_size: usize = catch_all_tier.read().unwrap().chunk_size;
        let mut catch_all_length = 0;  

        let mut seconds_passed:usize = 1;
        thread::sleep(Duration::from_secs(1));
        loop { 
            println!("Tick {} ", seconds_passed);  
            for tier in 0..=(num_tiers-2) {
                if seconds_passed % tier_vector[tier].read().unwrap().condition == 0 {
                    let tier_length = tier_vector[tier].read().unwrap().x_stats.len();
                    process_tier(&tier_vector[tier], &tier_vector[tier+1], tier_length)
                }
            }         
            {
            let mut catch_all_tier_write_lock = catch_all_tier.write().unwrap();            
            catch_all_length = catch_all_tier_write_lock.x_stats.len();
             
            if seconds_passed % CA_condition == 0 {
                merged_CA_last_x_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, true);
                merged_CA_last_y_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, false);
                //println!("Got the point {:?}", merged_CA_last_x_element);

                let mut tier_vector_write_lock = tier_vector[num_tiers-2].write().unwrap();
                //println!("The first elem of t2 was {:?}", tier_vector_write_lock.x_stats[0]);
                tier_vector_write_lock.x_stats[0] = merged_CA_last_x_element;
                tier_vector_write_lock.y_stats[0] = merged_CA_last_y_element;
                //println!("Now the first elem of t2 is {:?}", tier_vector_write_lock.x_stats[0]);
            }
            }
        catch_all_tier.write().unwrap().time_passed = Some(seconds_passed);
        seconds_passed += 1;
        thread::sleep(Duration::from_secs(1));
    }
});
}

pub fn interval_check_cut_no_ca(tier_vector :Vec<Arc<RwLock<TierData>>>, catch_all_tier: Arc<RwLock<TierData>>, num_tiers: usize) {
    println!("interval_check_cut_no_ca");
    thread::spawn(move || {
        let mut seconds_passed:usize = 1;
        thread::sleep(Duration::from_secs(1));
        //println!("First tier interval condition {}", tier_vector[0].read().unwrap().condition);
        loop {      
            //println!("Ticks {} ", seconds_passed);  
            for tier in 0..=(num_tiers-2) {
                if seconds_passed % tier_vector[tier].read().unwrap().condition == 0 {
                    let tier_length = tier_vector[tier].read().unwrap().x_stats.len();
                    process_tier(&tier_vector[tier], &tier_vector[tier+1], tier_length)
                }
            }          
        seconds_passed += 1;
        thread::sleep(Duration::from_secs(1));
    }
    });
}


pub fn count_check_cut_no_ca(tier_vector :Vec<Arc<RwLock<TierData>>>, catch_all_tier: Arc<RwLock<TierData>>, num_tiers: usize) {
    println!("count_check_cut_no_ca");
    thread::spawn(move || {
        let mut condition: usize;
        thread::sleep(Duration::from_secs(1));
        println!("First tier interval condition {}", tier_vector[0].read().unwrap().condition);
        loop {     
            for tier in 0..=(num_tiers-2) {
                condition = tier_vector[tier].read().unwrap().condition;
                /*THERE EXISTS AN EDGE CASE HAVE TO FIX - when launch the app, if the number of elements in the initial tier is greater than its cut condition before this thread is spawned
                the cut condition will never be met. A quick hack solution is to check if the length is greater than or equal to the condition. Proper fix is to block standard input reading until this thread created */
                if tier_vector[tier].read().unwrap().x_stats.len() >= condition {
                    println!("Tier merge {}", tier+1);
                    process_tier(&tier_vector[tier], &tier_vector[tier+1], condition);
                }
            }     
            thread::sleep(Duration::from_millis(1));     
        }
    });
} 

pub fn count_check_cut_ca(tier_vector :Vec<Arc<RwLock<TierData>>>, catch_all_tier: Arc<RwLock<TierData>>, num_tiers: usize) {
    println!("count_check_cut_ca");
    thread::spawn(move || { 
        let mut merged_ca_last_x_element;
        let mut merged_ca_last_y_element;
        let ca_condition = catch_all_tier.read().unwrap().condition;  
        let ca_chunk_size: usize = catch_all_tier.read().unwrap().chunk_size;
        let mut condition: usize;

        loop {
            //will break when only one tier, however this is an edge case, the Catch All only edge case
            for tier in 0..=(num_tiers-2) {  //only testing on first tier, initial tier, for now 
                condition = tier_vector[tier].read().unwrap().condition;
                if tier_vector[tier].read().unwrap().x_stats.len() == condition {
                    process_tier(&tier_vector[tier], &tier_vector[tier+1], condition);
                }
                
            } 

            thread::sleep(Duration::from_millis(1)); 

            let mut catch_all_tier_write_lock = catch_all_tier.write().unwrap();

            if catch_all_tier_write_lock.x_stats.len() == ca_condition {
                
                merged_ca_last_x_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size,ca_condition, true);
                merged_ca_last_y_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size,ca_condition, false);

                println!("After meging the tier it becomes");
                for bin in &catch_all_tier_write_lock.y_stats {
                    print!("{}, ", bin.mean);
                }
                println!("\n");
                println!("Got the point {:?}", merged_ca_last_y_element.mean);

                let mut tier_vector_write_lock = tier_vector[num_tiers-2].write().unwrap();
                println!("The first elem of t2 was {:?}", tier_vector_write_lock.y_stats[0].mean);
                tier_vector_write_lock.x_stats[0] = merged_ca_last_x_element;
                tier_vector_write_lock.y_stats[0] = merged_ca_last_y_element;
                println!("Now the first elem of t2 is {:?}", tier_vector_write_lock.y_stats[0].mean);
                println!("\n");
            }   
        }
    });
}

