use std::mem;
use eframe::epaint::mutex::RwLock;
use std::sync::Arc;

pub struct Tier {
    pub vec: Vec<[f64;2]>,
}

impl Tier {
    pub fn new() -> Self {
        Self { vec: vec![[0.0, 0.0]],}
    }

    pub fn push_float(&mut self, num: f64, x: f64) -> f64 {
        self.vec.push([x, num]);
        x + 2.0
    }

    pub fn get_points(&self) -> Vec<[f64;2]> {
        self.vec.clone().into_iter().collect()
    }

    pub fn get_length(&self) -> usize {
        self.vec.len()
    }

    pub fn calculate_average(&self,  values: &[[f64;2]]) -> [f64; 2] {
        //println!("Getting avg of : {:?}", values);

        let mut sum_x: f64 = 0.0;
        let mut sum_y: f64 = 0.0;
        let count = values.len() as f64;

        for &value in values {
            sum_x += value[0];
            sum_y += value[1];
        }

        let avg_x = if count > 0.0 { sum_x / count } else { 0.0 };
        let avg_y = if count > 0.0 { sum_y / count } else { 0.0 };

        [avg_x, avg_y]
    }

    pub fn get_y(&self) -> Vec<f64> {
        self.vec.iter().map(|&arr| arr[1]).collect()
    }

    /*
    pub fn merge_final_tier_vector_bins(&mut self, y: usize) -> [f64; 2] {
        println!("For the chunk {:?}", self.get_points());
        let mut tempBin: Vec<[f64; 2]> = Vec::new();
    
        // Iterate over the vector in chunks
        self.vec.chunks(y).for_each(|chunk| {
            let (sum_x, sum_y, count) = chunk.iter().fold((0.0, 0.0, 0), |(acc_x, acc_y, acc_count), &elem| {
                (acc_x + elem[0], acc_y + elem[1], acc_count + 1)
            });
    
            // Calculate the mean for x and y
            let mean_x = sum_x / count as f64;
            let mean_y = sum_y / count as f64;
    
            // Push the mean values to tempBin
            tempBin.push([mean_x, mean_y]);
    
            // Uncomment the println to log the details for each chunk
            println!("Chunk mean: x = {}, y = {}", mean_x, mean_y);
            

        });
        mem::replace(&mut self.vec, tempBin);
        self.vec[self.vec.len()-1]
    
        // You can now use tempBin for further processing
    }*/


    pub fn merge_final_tier_vector_bins_edge(&mut self, y: usize, length: usize, catch_all: &Arc<RwLock<Tier>>,)/* -> [f64; 2]*/ {

        let mut tempBin: Vec<[f64; 2]> = Vec::new();
        
        let selfVec = &self.vec;
        catch_all.write().vec.extend(selfVec);
        self.vec.drain(..length-1);
        let catch_all_length = catch_all.read().vec.len() - 1; //remember we exclude the final point of the catch all, it is the first element in the raw data vector

        println!("Catch all: {:?}", catch_all.read().get_y());
        println!("Catch all length: {:?}", catch_all.read().get_y().len());
        println!("R.D {:?}", self.get_y());

        
        catch_all.read().vec[0..catch_all_length].chunks(y).for_each(|chunk| {
            let (sum_x, sum_y, count) = chunk.iter().fold((0.0, 0.0, 0), |(acc_x, acc_y, acc_count), &elem| {
                (acc_x + elem[0], acc_y + elem[1], acc_count + 1)
            });
    
            // Calculate the mean for x and y
            let mean_x = sum_x / count as f64;
            let mean_y = sum_y / count as f64;
    
            // Push the mean values to tempBin
            tempBin.push([mean_x, mean_y]);

            println!("Chunk mean: y = {}", mean_y);
        });

        catch_all.write().vec.drain(0..catch_all_length);
        catch_all.write().vec.splice(0..0, tempBin);



    }

} 