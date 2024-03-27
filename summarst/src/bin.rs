#[derive(Debug, Clone, Default,Copy)] //allow deriving clones
pub struct Bin {
    pub mean: f64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
    pub range: f64,
    pub estimated_q1: f64,
    pub estimated_q3: f64,
}

impl Bin {
    pub fn new(mean: f64, sum: f64, min: f64, max: f64, count: usize) -> Self {

        let range = max - min;
        let estimated_q1 = mean - (0.25 * range);
        let estimated_q3 = mean + (0.25 * range);

        Bin {
            mean,
            sum,
            min,
            max,
            count,
            range,
            estimated_q1, // Exclude as per your request
            estimated_q3, // Exclude as per your request
        }
    }

    pub fn print(&self) {
        println!("Mean {}\nSum {}\n Min {}\nMax {}\nCount {}",self.mean, self.sum, self.min, self.max, self.count);
    }

    pub fn get_mean(&self) -> f64 {
        self.mean
    }

    pub fn create_uniform_bins(value: f64, number_of_bins: usize) -> Vec<Bin> {
        let mut bins = Vec::with_capacity(number_of_bins);

        for _ in 0..number_of_bins {
            let bin = Bin::new(value, value, value, value, 1);
            bins.push(bin);
        }

        bins
    }

}