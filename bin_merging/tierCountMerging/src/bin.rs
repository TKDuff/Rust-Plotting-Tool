#[derive(Debug, Clone, Default,Copy)] //allow deriving clones
pub struct Bin {
    pub mean: f64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
    pub sum_of_squares: f64,
    pub variance: f64,
    pub standard_deviation: f64,
}

impl Bin {
    fn print(&self) {
        println!("Mean {}\nSum {}\n Min {}\nMax {}\nCount {}Sum of Squares {}",self.mean, self.sum, self.min, self.max, self.count, self.sum_of_squares);
    }
    pub fn get_mean(&self) -> f64 {
        self.mean
    }
}