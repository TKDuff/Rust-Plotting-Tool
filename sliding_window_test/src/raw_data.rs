pub struct rawData {
    //each entry in value is array of two float, in the form [x,y]
    //f32, 2 to represent two values, x,y 
    pub values: Vec<[f64; 2]>,
    pub window_size: f64,
}

/*Implementation for measurements 
pub fn new() - think of a contstructor, creates instance of a type. This function returns new instance of Measurments, 'new' is the name of the function
Inside function {} 'values' field initialized to the default value of a VecDeque using the 'default()' method
*/

impl rawData {
    /* Associated function, 'new', creates intance of measurement type with 'values' field initialised with default type */
    pub fn new(window_size: f64) -> Self{
        Self { values: Vec::default(),
        window_size,
     }
    }

    pub fn append_value(&mut self, point: [f64; 2]) {
        self.values.push(point);
        //println!("{}", self.values.len());
        //println!("{}: {} {}", self.values.len(), point[0], point[1]);
        if self.values.len() % 50 == 0 {
            let mut sample: Vec<_> = self.values[0..50].to_vec();
            sample = self.down_sample(sample);
            //self.values.drain(0..50);
            self.values.splice(0..50, sample.into_iter());
            println!("---------------------------------");
            for value in &self.values {
                println!("{} {}" ,value[0], value[1]);
            }
        }
    }

    pub fn get_values(&self) -> Vec<[f64; 2]> {
        self.values.clone().into_iter().collect()
    }

    /*Takes in line string from standard input, converts two string numbers to float, appends them to the vector of points to plot*/
    pub fn append_str(&mut self, s:&str) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        self.append_value([parts[0].parse::<f64>().unwrap(), parts[1].parse::<f64>().unwrap()]);

    }

    
    pub fn down_sample(&mut self, mut clone_to_downsample: Vec<[f64; 2]>) -> Vec<[f64; 2]> {
        //println!("{} received", clone_to_downsample.len());
        for point in &mut clone_to_downsample {
            point[0] *= 2.0;
            point[1] *= 2.0;
        }
        clone_to_downsample
    }

}