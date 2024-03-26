
use std::{thread, usize};
use crossbeam::channel::{Receiver, Sender};
use crate::data_strategy::DataStrategy;
use crate::tier::TierData;
use std::sync::{Arc, RwLock};
use tokio::time::{self,Duration, Instant};
use crate::main_functions::process_tier;
use tokio::runtime::Runtime;
use tokio::io::{self, AsyncBufReadExt, BufReader};

/*
Spawn asynchronous task to
1) read stanard input lines and append them to stdin tier
2) check condition to aggregate points
Asynchronous task is non-blocking, allows concurrent appending of lines and condition checking
 */
pub fn create_count_stdin_read(rt: &Runtime, raw_data_thread: Arc<RwLock<dyn DataStrategy + Send + Sync>>, rd_sender: Sender<usize>) {
    rt.spawn(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        raw_data_thread.write().unwrap().append_str(line);      //append line
                        if let Some(cut_index) = raw_data_thread.write().unwrap().check_cut() {     //check condition to cut
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
/*
Asynchronous task, same as above except uses tokios 'tick' interval timer to tick every second
Upon launch waits the user set duration to launch, doesn't make sense to aggregate points straight away upon launch
 */
pub fn create_interval_stdin_read(rt: &Runtime, raw_data_thread: Arc<RwLock<dyn DataStrategy + Send + Sync>>, rd_sender: Sender<usize>) {
    let raw_data_interval_condition = raw_data_thread.read().unwrap().get_condition();

    let initial_delay = Duration::from_secs(raw_data_interval_condition as u64);
    let interval_duration = Duration::from_secs(raw_data_interval_condition as u64); 

    rt.spawn(async move {

        let mut interval = time::interval_at(Instant::now() + initial_delay, interval_duration);
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {      //append line
                        raw_data_thread.write().unwrap().append_str(line);  
                    } else {
                        break;
                    }
                }, 
                _ = interval.tick() => {
                    let length = raw_data_thread.read().unwrap().get_length();  //check condition to cut
                    rd_sender.send(length).unwrap();

                }
            }
        }
    });
}
//if no summarisation, the asynchronous task just appends lines, does not check the condition
pub fn crate_none_stdin_read(rt: &Runtime, raw_data_thread: Arc<RwLock<dyn DataStrategy + Send + Sync>>) {
    rt.spawn(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
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

    /*
    Thread that aggretates stdin tier chunk when message received, takes in
    - channel receiver
    - stind tier
    - initial tier (tier 1)

     */
pub fn create_raw_data_to_initial_tier(hd_receiver: Receiver<usize>, raw_data_accessor: Arc<RwLock<dyn DataStrategy + Send + Sync>>, initial_tier_accessor: Arc<RwLock<TierData>> )   {
    thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut aggregated_raw_data ; 
        for message in hd_receiver {
            {
                let mut aggregate_thread_raw_data_accessor_lock = raw_data_accessor.write().unwrap();   //aquire lock on stdin tier lock
                chunk = aggregate_thread_raw_data_accessor_lock.get_chunk(message); //get the chunk of points to be aggregated

                /*c
                call method to aggregate the x,y chunk into the x & y bin
                remember the last x,y value is excluded and will be first element in the initial tier
                method returns the newly created bins and last x,y value of chunk,both in form of bins*/   
                aggregated_raw_data = aggregate_thread_raw_data_accessor_lock.append_chunk_aggregate_statistics(chunk); 
                aggregate_thread_raw_data_accessor_lock.remove_chunk(message);  //remove the chunk of x,y values
            }

            {
                let mut initial_tier_lock = initial_tier_accessor.write().unwrap(); //aquire lock on initial tier
                let length = initial_tier_lock.x_stats.len() -1 ;  
                /*last element of initial tier is un-aggregated x,y value
                So 2nd last element is the actual newly created bins, thus they are pushed at 2nd last index (lenght -1)
                */
                initial_tier_lock.x_stats[length] = aggregated_raw_data.2;
                initial_tier_lock.y_stats[length] = aggregated_raw_data.3;

                //the un-aggregated x,y values returned from the 'append_chunk_aggregate_statistics()' method are in the form of bins, which are pushed to the initial tier
                initial_tier_lock.x_stats.push(aggregated_raw_data.0);
                initial_tier_lock.y_stats.push(aggregated_raw_data.1);
            }
        }   
    });
}


pub fn interval_rd_to_ca_edge(initial_tier_accessor: Arc<RwLock<TierData>>) {

    /*
    Edge case, just like count_rd_to_ca_edge
    If user only has stdin_data tier & catch-all-tier with no intermediate tiers this thread is spawned
    The catch-all-tier becomes the initial tier (tier 1)
    Since t1 contains a reference to the first element of the stdin_data tier, in this case the catch-all-tier contains a reference to the first point of stdin_data
    This catch-all-tier does not just contain bins, the final element is an un-aggregated x,y point from stdin_data
    Different to non-edge case catch all tier which only contains bins in its tier, the last element of it is the final bin of the tier. No reference to next tier.
    
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
                {
                let mut catch_all_tier_write_lock = initial_tier_accessor.write().unwrap();            
                catch_all_length = catch_all_tier_write_lock.x_stats.len()-1;
                catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, true);
                catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, false);
                }
            }
        initial_tier_accessor.write().unwrap().time_passed = Some(seconds_passed);
        seconds_passed += 1;
        thread::sleep(Duration::from_millis(1000));
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
    thread::spawn(move || {
        let mut catch_all_length: usize;
        thread::sleep(Duration::from_secs(1));
        let ca_condition = initial_tier_accessor.read().unwrap().condition;
        let ca_chunk_size: usize = initial_tier_accessor.read().unwrap().chunk_size;

        loop {
            catch_all_length = initial_tier_accessor.read().unwrap().x_stats.len()-1;
            if catch_all_length == ca_condition {
                {
                let mut catch_all_tier_write_lock = initial_tier_accessor.write().unwrap();            
                catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, true);
                catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, false);

                }
            }
        }
    });
}


/*Priority distpatcher method for interval
Iterates over each tier and checks condition constantly
If condition met, calls the merge method
 */
pub fn interval_check_cut_ca(tier_vector :Vec<Arc<RwLock<TierData>>>, catch_all_tier: Arc<RwLock<TierData>>, num_tiers: usize) {
    thread::spawn(move || {        
        let mut merged_ca_last_x_element;
        let mut merged_ca_last_y_element;
        let ca_condition = catch_all_tier.read().unwrap().condition;        //get catch all tier condition
        let ca_chunk_size: usize = catch_all_tier.read().unwrap().chunk_size;       //get catch all chunk size
        let mut catch_all_length: usize;  

        let mut seconds_passed:usize = 1;       //keep track of seconds passed, start at 1 second as don't merge upon launch
        thread::sleep(Duration::from_secs(1));
        loop { 
            for tier in 0..=(num_tiers-2) {                                                                     //for every intermediate tier
                if seconds_passed % tier_vector[tier].read().unwrap().condition == 0 {                                 //if their time condition is a factor of the seconds passed
                    let tier_length = tier_vector[tier].read().unwrap().x_stats.len();
                    process_tier(&tier_vector[tier], &tier_vector[tier+1], tier_length) //merge the tier bins into a single and push it to the next tier
                }
            }         
            {
            let mut catch_all_tier_write_lock = catch_all_tier.write().unwrap();    //aquire catch all tier lock           
            catch_all_length = catch_all_tier_write_lock.x_stats.len();
             
            if seconds_passed % ca_condition == 0 { //if catch all tier time condition met
                merged_ca_last_x_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, true);   //merge the x bins in chunks
                merged_ca_last_y_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size, catch_all_length, false);  //merge the y bins in chunks

                //replace the first element of the 2nd last tier with the last element of the now merged catch all tier (elements are bins in this case)
                let mut tier_vector_write_lock = tier_vector[num_tiers-2].write().unwrap();
                tier_vector_write_lock.x_stats[0] = merged_ca_last_x_element;
                tier_vector_write_lock.y_stats[0] = merged_ca_last_y_element;
            }
            }
        catch_all_tier.write().unwrap().time_passed = Some(seconds_passed); //record the seconds passed for the gui
        seconds_passed += 1;
        thread::sleep(Duration::from_millis(1000));
    }
});
}
//if there is no catch all policy, 0C0, then don't check the catch all tier condition
pub fn interval_check_cut_no_ca(tier_vector :Vec<Arc<RwLock<TierData>>>, catch_all_tier: Arc<RwLock<TierData>>, num_tiers: usize) {
    thread::spawn(move || {
        let mut seconds_passed:usize = 1;
        thread::sleep(Duration::from_secs(1));
        loop {      
            for tier in 0..=(num_tiers-2) {
                if seconds_passed % tier_vector[tier].read().unwrap().condition == 0 {
                    let tier_length = tier_vector[tier].read().unwrap().x_stats.len();
                    process_tier(&tier_vector[tier], &tier_vector[tier+1], tier_length)
                }
            } 
            
            catch_all_tier.write().unwrap().time_passed = Some(seconds_passed);         
            seconds_passed += 1;
            thread::sleep(Duration::from_secs(1));
        }
    });
}


pub fn count_check_cut_no_ca(tier_vector :Vec<Arc<RwLock<TierData>>>, num_tiers: usize) {
    thread::spawn(move || {
        let mut condition: usize;
        thread::sleep(Duration::from_secs(1));
        loop {     
            for tier in 0..=(num_tiers-2) {
                condition = tier_vector[tier].read().unwrap().condition;
                /*THERE EXISTS AN EDGE CASE HAVE TO FIX - when launch the app, if the number of elements in the initial tier is greater than its cut condition before this thread is spawned
                the cut condition will never be met. A quick hack solution is to check if the length is greater than or equal to the condition. Proper fix is to block standard input reading until this thread created */
                if tier_vector[tier].read().unwrap().x_stats.len() >= condition {
                    process_tier(&tier_vector[tier], &tier_vector[tier+1], condition);
                }
            }     
            thread::sleep(Duration::from_millis(1));     
        }
    });
} 
/*Priority dispatcher method for count
Works the same as interval_check_cut_ca() except
1) Doesn't keep track of time passed
2) Condition is if tier length is a factor of the tiers merge condition length
 */
pub fn count_check_cut_ca(tier_vector :Vec<Arc<RwLock<TierData>>>, catch_all_tier: Arc<RwLock<TierData>>, num_tiers: usize) {
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

            if catch_all_tier_write_lock.x_stats.len() >= ca_condition {
                merged_ca_last_x_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size,ca_condition, true);
                merged_ca_last_y_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(ca_chunk_size,ca_condition, false);

                let mut tier_vector_write_lock = tier_vector[num_tiers-2].write().unwrap();

                tier_vector_write_lock.x_stats[0] = merged_ca_last_x_element;
                tier_vector_write_lock.y_stats[0] = merged_ca_last_y_element;

            }   
        }
    });
}

