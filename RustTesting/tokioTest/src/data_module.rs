use std::usize;

use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};

pub struct StdinData {
    pub points: Vec<[f64;2]>,
}

impl StdinData {
    pub fn new() -> Self {
        //Self { points: Vec::default(),}
        Self { points: vec![[0.0, 0.0]],}
    }

    pub fn append_points(&mut self, points: [f64; 2]) {
        //println!("{} {}", points[0], points[1]);
        self.points.push([points[0], points[1]]);
        }

    /*Takes in line string from standard input, converts two string numbers to float, appends them to the vector of points to plot*/
    pub fn append_str(&mut self, s:&str) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        self.append_points([parts[0].parse::<f64>().unwrap(), parts[1].parse::<f64>().unwrap()]);
    }

    pub fn get_length(&self) -> usize {
        self.points.len() - 1
    }

    pub fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {
        self.points[1..count+1].to_vec()
    }

    /*So use into_iter if you want to consume the entire collection, and use drain if you only want to consume part of the collection or if you want to reuse the emptied collection later. */
    pub fn remove_chunk(&mut self, count:usize, point_means: (f64, f64)) {
        //println!("\nPre remove chunk {:?}", self.points);
        self.points[0] = [point_means.0, point_means.1];
        self.points.drain(1..count+1);
        //println!("Post remove chunk {:?}\n", self.points);
    }

    pub fn get_values(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }
}



#[derive(Clone, Default)] //allow deriving clones
pub struct Bin {
    pub mean: f64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
}
impl Bin {
    fn print(&self) {
        println!("Mean {}\nSum {}\n Min {}\nMax {}\nCount {}", self.mean, self.sum, self.min, self.max, self.count);
    }

    pub fn get_mean(&self) -> f64 {
        self.mean
    }
}

pub struct DownsampledData {
    pub x_stats: Vec<Bin>,
    pub y_stats: Vec<Bin>,
}
impl DownsampledData {
    pub fn new() -> Self {
        Self { 
            x_stats: Vec::new(),
            y_stats: Vec::new(),
        }
    }

    pub fn append_statistics(&mut self, chunk: Vec<[f64;2]>, point_count:usize) -> (f64, f64) {
        //println!("\nAggregating this chunk {:?}", chunk);
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().map(|&[x, y]| (x, y)).unzip(); //refactor, calculate mean while iterating over             
        
        
        let x_sum: f64 = x_vec.iter().sum();
        let y_sum: f64 = y_vec.iter().sum();
         
 
        let mut x = Data::new(x_vec.clone());   
        let mut y = Data::new(y_vec.clone());

        /*Look into using moving average */
        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        let aggregate_count = self.y_stats.len();

        self.x_stats.push(Bin { mean: x_mean, sum: x_sum, min: x.min(), max: x.max(), count: point_count });
        self.y_stats.push(Bin { mean: y_mean, sum: y_sum, min: y.min(), max: y.max(), count: point_count });
        //println!("The sum is: {} The lenght is: {}, The y mean is {}, The x mean is {}", y_sum, y.len(), y_mean, x_mean);

        if(self.y_stats.len() % 21 == 0) {
            println!("Length {}", self.y_stats.len());
            print!("Pre drain/slice: ");
            for i in 0..self.x_stats.len() {
                print!("{}, ", self.x_stats[i].mean);
            }
            println!("\n");


            let mut merged_y_stats = self.merge_vector_bins(&self.y_stats[0..aggregate_count], 2);
            let mut merged_x_stats = self.merge_vector_bins(&self.x_stats[0..aggregate_count], 2);

            println!("Merged x Stats:");
            for i in 0..merged_x_stats.len() {
                print!("{} ", merged_x_stats[i].get_mean());
            }

            self.y_stats.drain(0..aggregate_count);
            self.x_stats.drain(0..aggregate_count);

            self.x_stats.splice(0..0, merged_x_stats);
            self.y_stats.splice(0..0, merged_y_stats);

            println!("\nPost Drain: ");
            for i in 0..self.x_stats.len() {
                print!("{}, ", self.x_stats[i].mean);
            }
            println!("\n");
        }

        (x_mean, y_mean) //returned as replace aggregated chunk with with the average value, fills gap between two plots
    }


    pub fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }


    pub fn merge_vector_bins(&self, bins: &[Bin], y: usize) -> Vec<Bin> {
        let mut tempBin: Vec<Bin> = Vec::new();

        
        
        bins.chunks(y).for_each(|chunk| {
            // Calculate the sum and count for the current chunk
            let chunk_count: usize = chunk.iter().map(|bin| bin.count).sum();
            let chunk_sum: f64 = chunk.iter().map(|bin| bin.sum).sum();
            let chunk_min = chunk.iter().map(|bin| bin.min).fold(f64::INFINITY, f64::min);
            let chunk_max = chunk.iter().map(|bin| bin.max).fold(f64::NEG_INFINITY, f64::max);
            let chunk_mean = chunk_sum / chunk_count as f64;
            tempBin.push( Bin {mean: chunk_mean, sum: chunk_sum , min: chunk_min, max: chunk_max, count: chunk_count} );

            //println!("{} count: {} sum: {} min {} max {} SoS {} mean {}",cc, chunk_count, chunk_sum, chunk_min, chunk_max, chunk_sum_square, chunk_mean);
        });

        tempBin
    }

}





/*
    pub fn combineBins(&mut self) {

        
        let stat_bin_length = self.x_stats.len();
        let last_instances = stat_bin_length - 4;

        //println!("\nLength: {}\nLast Instance Index: {}", stat_bin_length, last_instances);
        
        let x_last_three: Vec<Bin>  = self.x_stats[last_instances..stat_bin_length-1].to_vec();
        let y_last_three: Vec<Bin> = self.y_stats[last_instances..stat_bin_length-1].to_vec();
        
        let combined_x = self.get_combined_stats(&x_last_three);
        let combined_y = self.get_combined_stats(&y_last_three);



        //println!("Last three: {:?}\nTo be inserted: {:?}\nFull: {:?}", x_last_three, combined_x, self.x_stats);
        self.x_stats[last_instances-1] = combined_x;
        self.y_stats[last_instances-1] = combined_y;
        self.x_stats.drain(last_instances..stat_bin_length-1);
        self.y_stats.drain(last_instances..stat_bin_length-1);
    }

    pub fn get_combined_stats(&self, stats: &Vec<Bin>) -> Bin {
        let total_count: usize = stats.iter().map(|x| x.count).sum();
        let total_sum: f64 = stats.iter().map(|x| x.sum).sum();

        
        //let combined_mean = stats.iter().map(|x| x.mean * x.count as f64).sum::<f64>() / total_count as f64;
        let combined_mean= total_sum / total_count as f64;

        let combined_stat = Bin {
            mean: combined_mean,
            sum: total_sum,
            min: 0.0,
            max: 0.0,
            count: total_count,
        };
        combined_stat
    }
    
    pub fn get_statistics (chunk: Vec<f64>) -> (f64, f64) {
        let sum: f64 = chunk.iter().sum();
        let data = Data::new(chunk.clone());
        let mean = data.mean().unwrap();
        (mean, sum)
    }
    
    */