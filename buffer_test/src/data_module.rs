use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};

pub struct StdinData {
    pub points: Vec<[f64;2]>,
}

impl StdinData {
    pub fn new() -> Self {
        Self { points: Vec::default(),}
    }

    pub fn append_points(&mut self, points: [f64; 2]) {
        self.points.push([points[0], points[1]]);
    }

    /*Takes in line string from standard input, converts two string numbers to float, appends them to the vector of points to plot*/
    pub fn append_str(&mut self, s:&str) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        self.append_points([parts[0].parse::<f64>().unwrap(), parts[1].parse::<f64>().unwrap()]);
    }

    pub fn get_length(&self) -> usize {
        self.points.len()
    }

    pub fn get_chunk(&self, start: usize, end: usize) -> Vec<[f64;2]> {
        self.points[start..end].to_vec()
    }

    /*So use into_iter if you want to consume the entire collection, and use drain if you only want to consume part of the collection or if you want to reuse the emptied collection later. */
    pub fn remove_chunk(&mut self, count:usize) {
        self.points.drain(0..count);
    }
}


pub struct DownsampledData {
    pub x_stats: Vec<[f64;6]>,
    pub y_stats: Vec<[f64;6]>,
}
impl DownsampledData {
    pub fn new() -> Self {
        Self { 
            x_stats: Vec::default(),
            y_stats: Vec::default(),
        }
    }

    pub fn append_statistics(&mut self, chunk: Vec<[f64;2]>) {
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().map(|&[x, y]| (x, y)).unzip();


        let mut x = Data::new(x_vec);
        let mut y = Data::new(y_vec);

        self.x_stats.push([x.lower_quartile(), x.upper_quartile(), x.median(), x.min(), x.max(), x.mean().unwrap()]);
        self.y_stats.push([y.lower_quartile(), y.upper_quartile(), y.median(), y.min(), y.max(), y.mean().unwrap()]);
    }
}