use std::collections::VecDeque;
use egui_plot::{Plot, PlotPoint};

pub struct Measurements {
    //each entry in value is array of two float, in the form [x,y]
    //f32, 2 to represent two values, x,y 
    pub values: VecDeque<[f64; 2]>
}

/*Implementation for measurements 
pub fn new() - think of a contstructor, creates instance of a type. This function returns new instance of Measurments, 'new' is the name of the function
Inside function {} 'values' field initialized to the default value of a VecDeque using the 'default()' method
*/

impl Measurements {
    /* Associated function, 'new', creates intance of measurement type with 'values' field initialised with default type */
    pub fn new() -> Self{
        Self { values: VecDeque::default(), }
    }

    pub fn append_value(&mut self, point: [f64; 2]) {
        self.values.push_back(point)

    }

    pub fn get_values(&self) -> Vec<[f64; 2]> {
        /*
        clone - create copy of values
        into_iter - converts values into iterator
        collect - 
         */
        self.values.clone().into_iter().collect()
    }
}