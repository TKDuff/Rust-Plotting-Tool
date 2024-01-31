use std::usize;
pub struct StdinData {
    pub points: Vec<[f64;2]>,
}

impl StdinData {
    pub fn new() -> Self {
        Self { points: vec![[0.0, 0.0]],}
    }

    pub fn append_points(&mut self, points: [f64; 2]) {
        self.points.push([points[0], points[1]]);
        }

    pub fn append_str(&mut self, line:String) {
        let values_result: Result<Vec<f64>, _> = line.split(' ')
        .map(|s| s.trim().parse::<f64>())
        .collect();

        match values_result {
            Ok(values) => {
                println!("{}", values[1]);
                self.points.push([values[0], values[1]]);
            }
            Err(err) => {
                println!("Error parsing values: {:?}", err);
            }
        }
    }

    pub fn get_length(&self) -> usize {
        self.points.len()
    }

    pub fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {
        println!("Here {} {}", count, self.points.len());
        self.points[..count].to_vec()
    }

    /*So use into_iter if you want to consume the entire collection, and use drain if you only want to consume part of the collection or if you want to reuse the emptied collection later. */
    pub fn remove_chunk(&mut self, count:usize, point_means: (f64, f64)) {
        self.points[0] = [point_means.0, point_means.1];
        self.points.drain(1..count+1);
    }


    pub fn get_values(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }
}

