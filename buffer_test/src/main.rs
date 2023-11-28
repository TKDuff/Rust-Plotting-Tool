mod data_module;
use data_module::{StdinData, DownsampledData};



use std::thread;
use std::io::{self, BufRead};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
static PROCESS_FLAG: AtomicBool = AtomicBool::new(false);

struct MyApp {
    raw_data: Arc<RwLock<StdinData>>,
    historic_data: Arc<RwLock<DownsampledData>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            raw_data: Arc::new(RwLock::new(StdinData::new())),
            historic_data: Arc::new(RwLock::new(DownsampledData::new())),
        }
    }
}

fn main() {
    let my_app = MyApp::default();
    let raw_data_thread = my_app.raw_data.clone();

    let raw_data_handle = thread::spawn(move || { 
        let stdin = io::stdin();          //global stdin instance
        let locked_stdin = stdin.lock();  //lock stdin for exclusive access
        

        for line in locked_stdin.lines() {
            let line_string = line.unwrap();
            raw_data_thread.write().unwrap().append_str(&line_string);
            if PROCESS_FLAG.load(Ordering::SeqCst) {
                println!("Remove chunk");
                raw_data_thread.write().unwrap().remove_chunk(50);
                PROCESS_FLAG.store(false, Ordering::SeqCst);
            }

        }
    });

    let downsampler_raw_data_thread = my_app.raw_data.clone();
    let downsampler_thread = my_app.historic_data.clone();


    let historic_data_handle = thread::spawn(move || {
        let mut prev_length = 0;
        let mut length = 0;
        let point_count = 50;
        let mut chunk: Vec<[f64;2]>;
        loop {
            length = downsampler_raw_data_thread.read().unwrap().get_length();
            if length % point_count == 0 && length != prev_length {
                chunk = downsampler_raw_data_thread.read().unwrap().get_chunk(length-point_count, length);
                downsampler_thread.write().unwrap().append_statistics(chunk);
                PROCESS_FLAG.store(true, Ordering::SeqCst);
                prev_length = length;
            }
        }
    });



    raw_data_handle.join().unwrap();
    historic_data_handle.join().unwrap();
}

//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/buffer_test/ && cargo run --bin buffer_test)