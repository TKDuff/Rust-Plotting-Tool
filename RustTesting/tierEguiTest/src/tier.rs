
pub struct Tier {
    pub vec: Vec<[f64;2]>,
}

impl Tier {
    pub fn new() -> Self {
        Self { vec: vec![[0.0, 0.0]],}
    }

    pub fn push_float(&mut self, num: f64, x: f64) -> f64 {
        self.vec.push([x, num]);
        x + 2.0
    }

    pub fn get_points(&self) -> Vec<[f64;2]> {
        self.vec.clone().into_iter().collect()
    }

    pub fn get_length(&self) -> usize {
        self.vec.len()
    }

    pub fn calculate_average(&self,  values: &[[f64;2]]) -> [f64; 2] {
        //println!("Getting avg of : {:?}", values);

        let mut sum_x: f64 = 0.0;
        let mut sum_y: f64 = 0.0;
        let count = values.len() as f64;

        for &value in values {
            sum_x += value[0];
            sum_y += value[1];
        }

        let avg_x = if count > 0.0 { sum_x / count } else { 0.0 };
        let avg_y = if count > 0.0 { sum_y / count } else { 0.0 };

        [avg_x, avg_y]
    }

    pub fn get_y(&self) -> Vec<f64> {
        self.vec.iter().map(|&arr| arr[1]).collect()
    }

}