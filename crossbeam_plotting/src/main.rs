use rayon::prelude::*;

fn main() {
    // Example: A list of vectors to downsample
    let data: Vec<Vec<i32>> = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        // Add more vectors as needed
    ];

    // Process each vector in parallel using rayon
    let downsampled: Vec<_> = data.into_par_iter()
        .map(|vec| downsample_vec(vec))
        .collect();

    // Output the results
    for vec in downsampled {
        println!("{:?}", vec);
    }
}
