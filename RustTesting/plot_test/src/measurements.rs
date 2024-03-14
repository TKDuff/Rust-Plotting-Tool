use std::collections::VecDeque;
use egui_plot::{Plot, PlotPoint};

pub struct Measurements {
    //each entry in value is array of two float, in the form [x,y]
    //f32, 2 to represent two values, x,y 
    pub values: VecDeque<[f64; 2]>,
    pub window_size: f64, 
}

/*Implementation for measurements 
pub fn new() - think of a contstructor, creates instance of a type. This function returns new instance of Measurments, 'new' is the name of the function
Inside function {} 'values' field initialized to the default value of a VecDeque using the 'default()' method
*/

impl Measurements {
    /* Associated function, 'new', creates intance of measurement type with 'values' field initialised with default type */
    pub fn new(window_size: f64) -> Self{
        Self { values: VecDeque::default(),
        window_size,
     }
    }

    pub fn append_value(&mut self, point: [f64; 2]) {
        self.values.push_back(point);

        /*dequeue anything less than window size from vector
        while let - loop to match right hand of expression with left side of expression, if match loop execute, if don't match loop exits
        Option<T> - Some(var_name) or None, used with while let to store values during/after loop execution
        Some(var_name) - will bind value (checking against) to var_name to use in the for loop
        None - if option return None, loop will exit
        

        Some() attemps to get front of 'values' vector, if front value exists it will be returned, stored in 'point' and while let loop executes
        If Some() gets the value 'None', while let loop won't execute*/

    }

    pub fn get_values(&self) -> Vec<[f64; 2]> {
        /*
        clone - create copy of values
        into_iter - converts values into iterator
        collect - check why I need this? 
         */
        self.values.clone().into_iter().collect()
    }

    /*Takes in line string from standard input, converts two string numbers to float, appends them to the vector of points to plot*/
    pub fn append_str(&mut self, s:&str) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        self.append_value([parts[0].parse::<f64>().unwrap(), parts[1].parse::<f64>().unwrap()]);

    }
}