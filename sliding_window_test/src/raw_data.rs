
#[derive(Default, Debug, Clone)]
pub struct Statistics {
    pub x_avg: f64,
    pub x_lower_quartile: f64,
    pub x_upper_quartile: f64,
    pub x_median: f64,
    pub x_min: f64,
    pub x_max: f64,

    pub y_avg: f64,
    pub y_lower_quartile: f64,
    pub y_upper_quartile: f64,
    pub y_median: f64,
    pub y_min: f64,
    pub y_max: f64,
}


pub struct RawData {
    //each entry in value is array of two float, in the form [x,y]
    //f32, 2 to represent two values, x,y 
    pub values: Vec<Statistics>,
    pub BoxPlotValue: Statistics,
}

/*Implementation for measurements 
pub fn new() - think of a contstructor, creates instance of a type. This function returns new instance of Measurments, 'new' is the name of the function
Inside function {} 'values' field initialized to the default value of a VecDeque using the 'default()' method
*/

impl RawData {
    /* Associated function, 'new', creates intance of measurement type with 'values' field initialised with default type */
    pub fn new() -> Self{
        Self { 
            values: Vec::new(),
            BoxPlotValue: Statistics {..Default::default()}
     }
    }
    
    pub fn get_plot_values(&self) -> Vec<[f64; 2]> {
        self.values.iter().map(|stat| [stat.x_avg, stat.y_avg]).collect()
    }
   
    pub fn get_chunk(&self, start:usize) -> (Vec<Statistics>, usize) {
        let len = self.values.len();
        (self.values[len - start..len].to_vec(), len)  //define end of vector, since new values being pushed in parralel must ensure don't include new values
    }

    pub fn append_chunk(&mut self, single_stat: Statistics, end_range: usize) {
        self.values.splice((end_range - 50)..end_range, std::iter::once(single_stat));
    }

    /*
    pub fn amend(&mut self, downsampled_section: &Vec<[f64; 2]>, start_range: usize, end_range: usize){
        self.values.splice(start_range..end_range, downsampled_section.iter().cloned());
    }*/

    /*Takes in line string from standard input, converts two string numbers to float, appends them to the vector of points to plot*/
    pub fn append_str(&mut self, s:&str) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        self.append_value([parts[0].parse::<f64>().unwrap(), parts[1].parse::<f64>().unwrap()]);
    }

    pub fn append_value(&mut self, point: [f64; 2]) {
        println!("{} {} {}", self.values.len(), point[0], point[1]);
        let new_stat = Statistics {
            x_avg: point[0],
            y_avg: point[1],
            ..Default::default()  // Other fields are set to their default values
        };
        self.values.push(new_stat);
    }

    pub fn append_box_plot(&mut self, stats: Statistics) {
        self.BoxPlotValue = stats;
        let boxPlot = &self.BoxPlotValue;
        println!("{} {} {} {}",  boxPlot.x_lower_quartile, boxPlot.x_upper_quartile, boxPlot.x_min, boxPlot.x_max );
    }

    pub fn get_box_plot (&self) -> Statistics {
        self.BoxPlotValue.clone()
    }

}

//create a struct with all the statistal values, store this in the 'values' vector
//writer just append the x,y values of the struct
//egui just plots the x,y values of the struct
//downsample takes chunk (so 50 structs, with empty values besides x,y)
//Downsamples x,y values of 50 structs
//replace with single struct, stats for x, stats for y
//this means
//           box plot is average of chunk
//           line plot is average of chunk, still showing live plots