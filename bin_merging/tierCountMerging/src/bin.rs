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
    pub range: f64,
    pub estimated_q1: f64,
    pub estimated_q3: f64,
}

impl Bin {
    pub fn new(mean: f64, sum: f64, min: f64, max: f64, count: usize, sum_of_squares: f64, variance: f64) -> Self {

        let range = max - min;
        let estimated_q1 = mean - (0.25 * range);
        let estimated_q3 = mean + (0.25 * range);
        let standard_deviation = variance.sqrt();

        Bin {
            mean,
            sum,
            min,
            max,
            count,
            sum_of_squares,
            variance,
            standard_deviation,
            range,
            estimated_q1, // Exclude as per your request
            estimated_q3, // Exclude as per your request
        }
    }

    pub fn print(&self) {
        println!("Mean {}\nSum {}\n Min {}\nMax {}\nCount {}Sum of Squares {}",self.mean, self.sum, self.min, self.max, self.count, self.sum_of_squares);
    }
    pub fn get_mean(&self) -> f64 {
        self.mean
    }


}