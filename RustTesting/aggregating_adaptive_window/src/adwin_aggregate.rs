pub struct ADWIN_aggregate {
    aggregate_points: Vec<[f64;2]>

}

// Implementation block for ADWIN
impl ADWIN_aggregate {
    // Constructor for creating a new ADWIN instance
    pub fn new() -> Self {
        ADWIN {
            aggregate_points: Vec::default(),
        }
    }

    //get_means
    pub fn get_aggregate_points(&self) -> Vec<[f64; 2]> {
        self.aggregate_points.clone().into_iter().collect()
    }



    pub fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) /*-> (f64, f64, usize)*/ {
           
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().skip(1).map(|&[x, y]| (x, y)).unzip();


        let mut x = Data::new(x_vec.clone());   
        let mut y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        let y_sum: f64 = y_vec.iter().sum();

        self.aggregate_points.push([x_mean, y_mean]);
        println!("\nx_Vec{:?}\ny_vec{:?}", x_vec, y_vec);
        println!("Method x mean is {} and y mean is {}\n", x_mean, y_mean);

        //(x_mean, y_mean, x.len())
    }


}
