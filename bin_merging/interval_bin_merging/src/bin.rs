#[derive(Debug, Clone, Default)] //allow deriving clones
pub struct Bin {
    pub mean: f64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
    pub timestamp: u128,
}

impl Bin {
    fn print(&self) {
        println!("Mean {}\nSum {}\n Min {}\nMax {}\nCount {}",self.mean, self.sum, self.min, self.max, self.count);
    }

    pub fn get_timestamp(&self) -> u128{
        self.timestamp
    }

    pub fn get_mean(&self) -> f64 {
        self.mean
    }
}