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

    pub fn append_value(&mut self, point: [f64; 2]) {
        println!("{} {} {}", self.values.len(), point[0], point[1]);
        self.values.push(point);

    }

    pub fn get_values(&self) -> Vec<[f64; 2]> {
        self.values.clone().into_iter().collect()
    }

    pub fn get_length(&self) -> usize {
        self.values.len()
    }

    pub fn get_previous_ten(&self, length:usize, length_minus_ten:usize) -> Vec<[f64; 2]> {
        self.values[length_minus_ten..length].to_vec()
    }

    pub fn amend(&mut self, amended_vector: &[f64; 2]){
        println!("Received");
    }

    /*Takes in line string from standard input, converts two string numbers to float, appends them to the vector of points to plot*/
    pub fn append_str(&mut self, s:&str) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        self.append_value([parts[0].parse::<f64>().unwrap(), parts[1].parse::<f64>().unwrap()]);

    }

}