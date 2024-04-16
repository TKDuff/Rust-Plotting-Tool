use crate::bin::Bin;

pub struct TierData {
    pub x_stats: Vec<Bin>,
    pub y_stats: Vec<Bin>,
    pub condition: usize,
    pub chunk_size: usize,
    pub time_passed: Option<usize>,
}

impl TierData {
    pub fn new(condition: usize, chunk_size: usize, time_passed: Option<usize>) -> Self {
        Self { 
            x_stats: vec![Bin::default()],
            y_stats: vec![Bin::default()],
            condition: condition,
            chunk_size: chunk_size,
            time_passed: time_passed,
        }
    }

    pub fn merge_vector_bins(&self, bins: &[Bin]/*, c: i32*/) -> Bin {

        let temp_bin;// Vec<Bin> = Vec::new();
        
        /* Calculate the sum and count for the current chunk
        - iterates over each bin in passed in slice
        - uses map to obtain specific field of bin, thus iterating over each bins particular field (min, max, count, sum)
        chatGPT was used to help find a way to find the minimum/maximum value for a certain field for a vector of instances (bins in this case) [1]
        */
        let chunk_count: usize = bins.iter().map(|bin| bin.count).sum();
        let chunk_sum: f64 = bins.iter().map(|bin| bin.sum).sum();
        let chunk_min = bins.iter().map(|bin| bin.min).fold(f64::INFINITY, f64::min); //using .fold to apply f64::min function find smallest value 
        let chunk_max = bins.iter().map(|bin| bin.max).fold(f64::NEG_INFINITY, f64::max); //using .fold to find maximum value 
        let chunk_mean = chunk_sum / chunk_count as f64;    //mean is the mean of the combined bins (sum/count), not the mean of means


        temp_bin =  Bin::new(chunk_mean, chunk_sum ,chunk_min, chunk_max, chunk_count);     
        temp_bin 
    }

    pub fn push_x_bin_vec(& mut self, x_bins: Vec<Bin>) {
        self.x_stats.extend(x_bins);
    }

    pub fn push_y_bin_vec(& mut self, y_bins: Vec<Bin>) {
        self.y_stats.extend(y_bins);
    }

    pub fn get_slices(&self, length: usize) -> (&[Bin], &[Bin])  {
        /*using 'cmp' to ensure slice not out of bounds
        start at 2nd element (exclude first bin) up to the 2nd last element (excluding last bin)
        Compares 'length' value to the tier length minux 1 and chooses whichever is smaller, ensuring the slice is not out of bounds
        */
        let x_slice = &self.x_stats[1..std::cmp::min(length, self.x_stats.len() - 1)];
        let y_slice = &self.y_stats[1..std::cmp::min(length, self.y_stats.len() - 1)];

        (x_slice, y_slice)
    }

    pub fn update_chunk_size(&mut self, new_size: usize) {
        self.chunk_size = new_size;
    }

    //catch all tier merge bins in chunks
    pub fn merge_final_tier_vector_bins(&mut self, chunk_size: usize,length: usize,  x: bool) -> Bin {
        
        let to_merge = if x {&mut self.x_stats} else {&mut self.y_stats};       

        /*this chunking does introduce a rounding error since the mean is the sum divided by the count, done each time again and again introduce the floating error
        uses the .chunks method to split the vector in chunks of a specified size
        Any remainder bins not included in a defined chunk are merged into a single bin
        */
        let temp_bins = to_merge[..length].chunks(chunk_size).map(|chunk| {
            let chunk_count: usize = chunk.iter().map(|bin| bin.count).sum();
            let chunk_sum: f64 = chunk.iter().map(|bin| bin.sum).sum();
            let chunk_min = chunk.iter().map(|bin| bin.min).fold(f64::INFINITY, f64::min);
            let chunk_max = chunk.iter().map(|bin| bin.max).fold(f64::NEG_INFINITY, f64::max);
            let chunk_mean:f64 = if chunk_count > 0 { chunk_sum / chunk_count as f64 } else { 0.0 };

            Bin::new(chunk_mean, chunk_sum, chunk_min, chunk_max, chunk_count)
        }).collect::<Vec<Bin>>();   //cannot infer iterator is collecting into a Bin struct,have to explicitaly tell it to collect into Vector of Bins


        to_merge.drain(0..length);
        to_merge.splice(0..0, temp_bins);
        to_merge[to_merge.len()-1]        
    }

    pub fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }


    /*If x_plots true return attributes in order to create box plot 
    Notice, how the x_stats is returned as the final value in both cases. This is specific scenario when creating the y_stats box plots
    Since y values can be the same value, it means the plots will be ontop of each other.
    To ensrue y box plots not overlapping, will position them using the x_stats means, which do not overlap
    Since the y box plots have x co-ord of x_stats means and y co-ord of y stats mean, should be like a line plot
    */
    pub fn get_box_plot_stats(&self) -> Vec<(f64, f64, f64, f64, f64, f64)> {
        // When x_plots is false, use y_stats for the stats but include x_stats mean as the last element
        self.y_stats.iter().zip(self.x_stats.iter())
            .map(|(y_bin, x_bin)| (y_bin.mean, y_bin.min, y_bin.max, y_bin.estimated_q1, y_bin.estimated_q3, x_bin.mean))
            .collect()
        }
        
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_vector_bins() {
        
        let bins = vec![
            Bin::new(-10.527, 150.754, -20.321, 0.125, 15),
            Bin::new(30.214, 60.468, 25.135, 35.356, 2),
            Bin::new(100.125, 200.256, 90.432, 110.654, 2),
            Bin::new(-5.543, -55.786, -15.214, 4.678, 10),
            Bin::new(500.786, 1000.321, 450.543, 550.876, 2),
            Bin::new(0.753, 15.784, -1.987, 2.546, 20),
            Bin::new(-100.432, -200.876, -150.654, -50.123, 2),
            Bin::new(200.654, 400.789, 150.321, 250.987, 2),
            Bin::new(1.345, 10.786, 0.543, 1.678, 10),
            Bin::new(3000.876, 6000.321, 2500.654, 3500.789, 2),
            
        ];

        let tier_data = TierData::new(0, 0, None);
        let merged_bin = tier_data.merge_vector_bins(&bins);

        assert_eq!(merged_bin.mean,  113.17637313432836);
        assert_eq!(merged_bin.sum, 7582.817);
        assert_eq!(merged_bin.count, 67); 
        assert_eq!(merged_bin.min, -150.654);
        assert_eq!(merged_bin.max, 3500.789);
    }

    #[test]
    fn test_merge_final_tier_vector_bins() {
        let bins = vec![
            Bin::new(-10.527, 150.754, -20.321, 0.125, 15),
            Bin::new(30.214, 60.468, 25.135, 35.356, 2),
            Bin::new(100.125, 200.256, 90.432, 110.654, 2),
            Bin::new(-5.543, -55.786, -15.214, 4.678, 10),
            Bin::new(500.786, 1000.321, 450.543, 550.876, 2),
            Bin::new(0.753, 15.784, -1.987, 2.546, 20),
            Bin::new(-100.432, -200.876, -150.654, -50.123, 2),
            Bin::new(200.654, 400.789, 150.321, 250.987, 2),
            Bin::new(1.345, 10.786, 0.543, 1.678, 10),
            Bin::new(3000.876, 6000.321, 2500.654, 3500.789, 2),    
        ];  

        let mut tier_data = TierData::new(0, 0, None);
        tier_data.x_stats = bins.clone();
        tier_data.y_stats = bins;
        let merged_x_bin = tier_data.merge_final_tier_vector_bins(3, 10, true); //test for merging the tier x bin vector

        //remember, the mean is Mean of Merged Bins, not mean of means
        assert!((tier_data.x_stats[0].mean - 21.656).abs() < 1e-3);
        assert!((tier_data.x_stats[0].sum - 411.477).abs() < 1e-3);
        assert_eq!(tier_data.x_stats[0].min, -20.321);
        assert_eq!(tier_data.x_stats[0].max, 110.654);
        assert_eq!(tier_data.x_stats[0].count, 19);
        assert!((tier_data.x_stats[0].range - 130.975).abs() < 1e-3);
        assert!((tier_data.x_stats[0].estimated_q1 - -11.0870).abs() < 1e-3);
        assert!((tier_data.x_stats[0].estimated_q3 - 54.4004).abs() < 1e-3);

        //values should be the same for the y bin of the tier, double checking works on y bin
        assert!((tier_data.y_stats[0].mean - 21.656).abs() < 1e-3);
        assert!((tier_data.y_stats[0].sum - 411.477).abs() < 1e-3);
        assert_eq!(tier_data.y_stats[0].min, -20.321);
        assert_eq!(tier_data.y_stats[0].max, 110.654);
        assert_eq!(tier_data.y_stats[0].count, 19);
        assert!((tier_data.y_stats[0].range - 130.975).abs() < 1e-3);
        assert!((tier_data.y_stats[0].estimated_q1 - -11.0870).abs() < 1e-3);
        assert!((tier_data.y_stats[0].estimated_q3 - 54.4004).abs() < 1e-3);

        //10 in chunks of 3 results in 3 with remainder 1, so 4 elements
        assert_eq!(tier_data.x_stats.len(), 4);
        assert_eq!(tier_data.y_stats.len(), 4);

        assert_eq!(tier_data.y_stats[3].count, 2);

        //ensure the final element of the now merged tier vector is the remainder. 10 into chunks of 3 leave remainder 1 un-merged
        assert_eq!(merged_x_bin.mean, 3000.1605); //although the mean ends in .876, the sum/count mean remainder is .1605
        assert_eq!(merged_x_bin.sum, 6000.321);
        assert_eq!(merged_x_bin.min, 2500.654);
        assert_eq!(merged_x_bin.max, 3500.789);
        assert_eq!(merged_x_bin.count, 2);
        assert!((merged_x_bin.range - 1000.135).abs() < 1e-3);
        assert!((merged_x_bin.estimated_q1 - 2750.126).abs() < 1e-3);
        assert!((merged_x_bin.estimated_q3 - 3250.194).abs() < 1e-3);


    }

}

/*
[1] - ChatGPT version 4, default setting, prompt "For a vector of the 'Bin' instances, how can the minimum value out of all the 'minimum' attributes be found?"
*/