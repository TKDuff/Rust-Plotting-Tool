pub struct PlotOne {
    pub values: Vec<[f64;2]>,
}

impl PlotOne {
    pub fn new() -> Self{
        Self { values: Vec::default(),}
    }

    pub fn append_value(&mut self, point_one: f64, point_two: f64) {
        self.values.push([point_one, point_two]);
    }

    /*
    pub fn get_values(&self) -> Vec<[f64; 2]> {
        self.values.clone().into_iter().collect()
    }*/

    pub fn get_chunk(&self, points: usize) -> Vec<[f64; 2]> {
        let len = self.values.len();
        self.values[(len - points)..].to_vec()
    }
}