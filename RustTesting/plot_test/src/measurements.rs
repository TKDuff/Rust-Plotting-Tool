use std::collections::VecDeque;
use egui_plot::{Plot, PlotPoint};
extern crate lttb;

use lttb::{DataPoint,lttb};

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
        if let Some(last) = self.values.back() {
            if last[0] >= point[0] {
                self.values = VecDeque::default();
            }
        }
        let min_x = point[0] - self.window_size;
        self.values.push_back(point);

        /*dequeue anything less than window size from vector
        while let - loop to match right hand of expression with left side of expression, if match loop execute, if don't match loop exits
        Option<T> - Some(var_name) or None, used with while let to store values during/after loop execution
        Some(var_name) - will bind value (checking against) to var_name to use in the for loop
        None - if option return None, loop will exit
        

        Some() attemps to get front of 'values' vector, if front value exists it will be returned, stored in 'point' and while let loop executes
        If Some() gets the value 'None', while let loop won't execute*/
        while let Some(point) = self.values.front() {
            if point[0] < min_x {
                self.values.pop_front();
            } else {
                break;
            }
        }

    }

    pub fn get_values(&self) -> Vec<[f64; 2]> {
        /*
        clone - create copy of values
        into_iter - converts values into iterator
        collect - check why I need this? 
        self.values.clone().into_iter().collect()*/
        vec![[2.0,5.0], [3.0,9.0], [4.0,6.0], [5.0,18.0], [7.0,12.0],[8.0,15.0],[9.0,14.0],
             [10.0,7.0], [12.0,9.0], [14.0,12.0], [15.0,18.0], [17.0,22.0],[20.0,15.0],[21.0,14.0]
        ]
    }

    pub fn get_lttb(&self) -> Vec<[f64; 2]> {
        let mut raw = vec!();
        let points = vec![(2.0,5.0), (3.0,9.0), (4.0,6.0), (5.0,18.0), (7.0,12.0),(8.0,15.0),(9.0,14.0),
        (10.0,7.0), (12.0,9.0), (14.0,12.0), (15.0,18.0), (17.0,22.0),(20.0,15.0),(21.0,14.0),];

        for(p1, p2) in points {
            raw.push(DataPoint::new(p1, p2));
          }

        let downsampled = lttb(raw, 7);
        //let tuples: Vec<(f64, f64)> = downsampled.iter().map(|dp| (dp.x, dp.y)).collect();
        let arrays: Vec<[f64; 2]> = downsampled.iter().map(|dp| [dp.x, dp.y]).collect();
        arrays

    }


    /*Takes in line string from standard input, converts two string numbers to float, appends them to the vector of points to plot*/
    pub fn append_str(&mut self, s:&str) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        self.append_value([parts[0].parse::<f64>().unwrap(), parts[1].parse::<f64>().unwrap()]);

    }
}