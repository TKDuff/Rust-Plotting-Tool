use crate::data_strategy::DataStrategy;

pub struct CountRawData {
    pub points_count: usize,
    pub points: Vec<[f64;2]>,
}

impl CountRawData {
    pub fn new() -> Self {
        Self { 
            points_count: 20,
            points: vec![[0.0, 0.0]]
        }
    }

}

impl DataStrategy for CountRawData {


    fn append_str(&mut self, line:String) {
        let values_result: Result<Vec<f64>, _> = line.split(' ')
        .map(|s| s.trim().parse::<f64>())
        .collect();

        match values_result {
            Ok(values) => {
                self.append_point(values[0], values[1]);
            }
            Err(err) => {
                println!("Error parsing values: {:?}", err);
            }
        }
    }

    fn get_raw_data(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }

    fn append_point(&mut self, x_value: f64, y_value: f64) {
        self.points.push([x_value, y_value]);
    }

    fn requires_external_trigger(&self) -> bool {
        false
    }

    fn get_values(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }

    fn remove_chunk(&mut self, count:usize, point_means: (f64, f64)) {
        /*
        As reminder, the raw data vector keeps reference as the calculated mean as the first element
        - Done since maintain plot continuity from tier 1 vector to raw data 'points' vector
        - I.E points = [A,B,C,D,E,F] then on merge points = [AE, F] and tier 1 = [AE]
        - As plot, distinct where aggregate plot is, AE, and where new plot begins F. 
        - To ensure conncted this creates a line from AE to F, think of as [AE -> AE -> F]
        - Tier 1 only contains mean, no raw data, thus distinct in its purpose and last point in its plot is seen as mean
        - More clarity than if Tier 1 was [AE, F] upon merge, that means second last element of vector is the actual mean
        */
        println!("Points mean x:{} y:{}", point_means.0, point_means.1);
        self.points[0] = [point_means.0, point_means.1];
        self.points.drain(1..count+1);
    }

    fn check_cut(&self) -> Option<usize> {
        if (self.points.len() - 1) % self.points_count == 0 {
            return Some(self.points.len() -1 )
        } else {
            None
        }
    }

    fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {
        self.points[1..count+1].to_vec()
    }
}