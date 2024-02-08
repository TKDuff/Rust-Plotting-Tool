use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};

#[derive(Debug, Clone, Default)] //allow deriving clones
pub struct Bin {
    pub mean: f64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
    pub sum_square: f64,
}
impl Bin {
    pub fn print(&self) {
        println!("Mean {}\nSum {}\n Min {}\nMax {}\nCount {}",self.mean, self.sum, self.min, self.max, self.count);
    }

    pub fn population_variance(&self) ->f64 {
        self.sum_square / self.count as f64
    }

    pub fn standard_deviation(&self) -> f64 {
        self.population_variance().sqrt()
    }

    pub fn get_mean(&self) -> f64 {
        self.mean
    }
}

pub struct AggregateData {
    pub x_stats: Vec<Bin>,
    pub y_stats: Vec<Bin>,
}

impl AggregateData {
    pub fn new() -> Self {
        Self { 
            x_stats: Vec::new(),
            y_stats: Vec::new(),
        }
    }

    pub fn calculate_sum_of_squares(&self, data: &[f64], mean: f64) -> f64 {
        data.iter().map(|&value| (value - mean).powi(2)).sum()
    }


    pub fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (f64, f64, usize) {
            
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().skip(1).map(|&[x, y]| (x, y)).unzip();


        let mut x = Data::new(x_vec.clone());   
        let mut y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        let x_sum: f64 = y_vec.iter().sum();

        let y_SoS= self.calculate_sum_of_squares(&y_vec, y_mean);
        let x_SoS= self.calculate_sum_of_squares(&y_vec, y_mean);

        let aggregate_count = self.y_stats.len();

        self.x_stats.push(Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len(), sum_square: x_SoS });
        self.y_stats.push(Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len(), sum_square: y_SoS });

        println!("\nY:\nsum: {}\nlength: {},\nmean {},\nSos {}\nVariance: {}\nStdDev: {}", x_sum, x.len(), x_mean, x_SoS, self.x_stats[aggregate_count].population_variance(), self.x_stats[aggregate_count].standard_deviation());

        //println!("Number of y_stats bins {}\n", aggregate_count);

        if(self.y_stats.len() % 10 == 0) {

            print!("Pre drain/slice: ");
            for i in 0..self.x_stats.len() {
                print!("{}, ", self.x_stats[i].mean);
            }
            println!("\n");

            let mut merged_y_stats = self.merge_vector_bins(&self.y_stats[0..aggregate_count], 3, 0);
            self.y_stats.drain(0..aggregate_count);



            let mut merged_x_stats = self.merge_vector_bins(&self.x_stats[0..aggregate_count], 3, 1);
            self.x_stats.drain(0..aggregate_count);

            println!("Merged Stats:");
            for i in 0..merged_x_stats.len() {
                print!("{} ", merged_x_stats[i].get_mean());
            }
            
            self.x_stats.splice(0..0, merged_x_stats);
            self.y_stats.splice(0..0, merged_y_stats);
            
            println!("\nPost Drain: ");
            for i in 0..self.x_stats.len() {
                print!("{}, ", self.x_stats[i].mean);
            }
            println!("\n");
        }


        (x_mean, y_mean, x.len())
    }

    pub fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }

    pub fn merge_vector_bins(&self, bins: &[Bin], y: usize, cc: i32) -> Vec<Bin> {
        let mut tempBin: Vec<Bin> = Vec::new();

        
        
        bins.chunks(y).for_each(|chunk| {
            // Calculate the sum and count for the current chunk
            let chunk_count: usize = chunk.iter().map(|bin| bin.count).sum();
            let chunk_sum: f64 = chunk.iter().map(|bin| bin.sum).sum();
            let chunk_min = chunk.iter().map(|bin| bin.min).fold(f64::INFINITY, f64::min);
            let chunk_max = chunk.iter().map(|bin| bin.max).fold(f64::NEG_INFINITY, f64::max);
            let chunk_sum_square: f64 = chunk.iter().map(|bin| bin.sum_square as f64).sum();
            let chunk_mean = chunk_sum / chunk_count as f64;
            tempBin.push( Bin {mean: chunk_mean, sum: chunk_sum , min: chunk_min, max: chunk_max, count: chunk_count, sum_square: chunk_sum_square } );

            /*
            println!("\n");
            for bin in chunk {
                print!("{},", bin.get_mean());
            }*/

            //println!("{} count: {} sum: {} min {} max {} SoS {} mean {}",cc, chunk_count, chunk_sum, chunk_min, chunk_max, chunk_sum_square, chunk_mean);
        });

        tempBin
    }

    
}