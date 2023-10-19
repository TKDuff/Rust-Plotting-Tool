use std::collections::VecDeque;
use egui_plot::{Plot, PlotPoint};

pub struct Measurements {
    //each entry in value is array of two float, in the form [x,y]
    pub values: VecDeque<[f32; 2]>
}

/*Implementation for measurements 
pub fn new() - think of a contstructor, creates instance of a type. This function returns new instance of Measurments, 'new' is the name of the function
Inside function {} 'values' field initialized to the default value of a VecDeque using the 'default()' method
*/

impl Measurements {
    pub fn new() -> Self{
        Self { values: VecDeque::default(), }
    }
}