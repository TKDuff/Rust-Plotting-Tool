pub struct RawData {
    //each entry in value is array of two float, in the form [x,y]
    //f32, 2 to represent two values, x,y 
    pub values: Vec<[f64; 2]>,
    pub window_size: f64,
}

/*Implementation for measurements 
pub fn new() - think of a contstructor, creates instance of a type. This function returns new instance of Measurments, 'new' is the name of the function
Inside function {} 'values' field initialized to the default value of a VecDeque using the 'default()' method
*/

impl RawData {
    /* Associated function, 'new', creates intance of measurement type with 'values' field initialised with default type */
    pub fn new(window_size: f64) -> Self{
        Self { values: Vec::default(),
        window_size,
     }
    }
    
    pub fn get_values(&self) -> Vec<[f64; 2]> {
        self.values.clone().into_iter().collect()
    }

    pub fn get_chunk(&self, start:usize) -> Vec<[f64; 2]> {
        let len = self.values.len();
        self.values[len - start..len].to_vec()  //define end of vector, since new values being pushed in parralel must ensure don't include new values
    }

    pub fn amend(&mut self, downsampled_section: &Vec<[f64; 2]>, start_range: usize, end_range: usize){
        self.values.splice(start_range..end_range, downsampled_section.iter().cloned());
    }

    /*Takes in line string from standard input, converts two string numbers to float, appends them to the vector of points to plot*/
    pub fn append_str(&mut self, s:&str) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        self.append_value([parts[0].parse::<f64>().unwrap(), parts[1].parse::<f64>().unwrap()]);
    }

    pub fn append_value(&mut self, point: [f64; 2]) {
        println!("{} {} {}", self.values.len(), point[0], point[1]);
        self.values.push(point);
    }

}