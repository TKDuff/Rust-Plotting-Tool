use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};

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
        println!("Mean {}\nSum {}\n Min {}\nMax {}\nCount {}",self.mean, self.sum, self.min, self.max, self.count);
    }
}
pub struct ADWIN_aggregate {
    //aggregate_points: Vec<[f64;2]>
    pub x_stats: Vec<Bin>,
    pub y_stats: Vec<Bin>,
}

// Implementation block for ADWIN
impl ADWIN_aggregate {
    // Constructor for creating a new ADWIN instance
    pub fn new() -> ADWIN_aggregate {
        ADWIN_aggregate { 
            x_stats: Vec::new(),
            y_stats: Vec::new(),
        }
    }

    //get_means
    pub fn get_aggregate_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }



    pub fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>, cut_index:usize) -> (f64, f64, usize) {
           
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().skip(1).map(|&[x, y]| (x, y)).unzip();


        let x = Data::new(x_vec.clone());   
        let y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();


        self.x_stats.push(Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len() });
        self.y_stats.push(Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len() });


        println!("\nx_Vec{:?}\ny_vec{:?}", x_vec, y_vec);
        println!("Method x mean is {} and y mean is {}\n", x_mean, y_mean);

        (x_mean, y_mean, cut_index)
    }


}
