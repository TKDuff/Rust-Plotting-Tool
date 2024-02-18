
use crate::bin::Bin;

pub struct TierData {
    pub x_stats: Vec<Bin>,
    pub y_stats: Vec<Bin>,
}

impl TierData {
    pub fn new() -> Self {
        Self { 
            x_stats: Vec::new(),
            y_stats: Vec::new(),
        }
    }

    pub fn merge_vector_bins(&self, bins: &[Bin]/*, c: i32*/) -> Vec<Bin> {

        if self.x_stats.is_empty() || (self.x_stats.len() == 1) {
            println!("x_stats is empty!");
        }
        
        
        // if( c== 1){  
        // print!("Pre merge Vector:");
        //     for bin in &self.x_stats {
        //         print!("{}, ", bin.get_mean());
        //     }
        // }

        // println!("");
        // if( c== 1){   
        //     print!("Pre merge chunk: ");
        //     for bin in bins {
        //         print!("{}, ", bin.get_mean());
        //     }
        // }

        let mut temp_bin: Vec<Bin> = Vec::new();
        
        // Calculate the sum and count for the current chunk
        let chunk_count: usize = bins.iter().map(|bin| bin.count).sum();
        let chunk_sum: f64 = bins.iter().map(|bin| bin.sum).sum();
        let chunk_min = bins.iter().map(|bin| bin.min).fold(f64::INFINITY, f64::min);
        let chunk_max = bins.iter().map(|bin| bin.max).fold(f64::NEG_INFINITY, f64::max);
        let chunk_mean = chunk_sum / chunk_count as f64;
        temp_bin.push( Bin {mean: chunk_mean, sum: chunk_sum , min: chunk_min, max: chunk_max, count: chunk_count} );

        //println!("{} count: {} sum: {} min {} max {} SoS {} mean {}",cc, chunk_count, chunk_sum, chunk_min, chunk_max, chunk_sum_square, chunk_mean);
            
        // println!("");
        // if( c== 1){   
        //     print!("Post merge chunk:");
        //     for bin in &tempBin {
        //         print!("{}, ", bin.get_mean());
        //     }
        // }

        
        temp_bin 
    }

    pub fn push_x_bin_vec(& mut self, x_bins: Vec<Bin>) {
        self.x_stats.extend(x_bins);
    }

    pub fn push_y_bin_vec(& mut self, y_bins: Vec<Bin>) {
        self.y_stats.extend(y_bins);
    }

    pub fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }

    pub fn print_x_means(&self) {
        print!("X Means:");
        for bin in &self.x_stats {
            print!("{:.2}, ", bin.mean);
        }
    }

    pub fn print_y_means(&self) {
        print!("Y Means:");
        for bin in &self.y_stats {
            print!("{:.2}, ", bin.mean);
        }
    }

    pub fn drain_x_vector(& mut self, range: usize) {
        self.x_stats.drain(0..range);

        print!("Post merge Vector:");
        for bin in &self.x_stats {
            print!("{}, ", bin.get_mean());
        }
        println!("");   
    }
}