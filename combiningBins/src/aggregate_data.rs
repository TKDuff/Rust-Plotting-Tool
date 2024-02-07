use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};

#[derive(Clone, Default)] //allow deriving clones
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

    pub fn population_variance(&self) -> f64 {
        if self.count > 0 {
            (self.sum_square / self.count as f64) - (self.mean.powi(2))
        } else {
            0.0
        }
    }

    pub fn standard_deviation(&self) -> f64 {
        self.population_variance().sqrt()
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

        let y_sum: f64 = y_vec.iter().sum();

        let y_SoS= self.calculate_sum_of_squares(&y_vec, y_mean);
        let x_SoS= self.calculate_sum_of_squares(&y_vec, y_mean);

        let aggregate_count = self.y_stats.len();
        

        self.x_stats.push(Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len(), sum_square: x_SoS });
        self.y_stats.push(Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len(), sum_square: y_SoS });

        println!("\n\nFor the vector{:?}", y_vec);
        println!("\nY:\nsum: {}\nlength: {},\nmean {},\nSos {}\nVariance: {}\nStdDev: {}\n", y_sum, y.len(), y_mean, y_SoS, self.y_stats[aggregate_count-1].population_variance(), self.y_stats[aggregate_count].standard_deviation());

        (x_mean, y_mean, x.len())
    }

    pub fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }

    
}