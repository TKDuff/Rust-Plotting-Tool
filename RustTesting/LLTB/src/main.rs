extern crate lttb;

use lttb::{DataPoint,lttb};

fn main() {
  let mut raw = vec!();
  let points = vec![(2.0,5.0), (3.0,9.0), (4.0,6.0), (5.0,18.0), (7.0,12.0),(8.0,15.0),(9.0,14.0),
                    (10.0,7.0), (12.0,9.0), (14.0,12.0), (15.0,18.0), (17.0,22.0),(20.0,15.0),(21.0,14.0),];

  /*
  raw.push(DataPoint::new(0.0, 10.0));
  raw.push(DataPoint::new(1.0, 12.0));
  raw.push(DataPoint::new(2.0, 8.0));
  raw.push(DataPoint::new(3.0, 10.0));
  raw.push(DataPoint::new(4.0, 12.0));*/
  for(p1, p2) in points {
    raw.push(DataPoint::new(p1, p2));
  }

  // Downsample the raw data to use just three datapoints.
  let downsampled = lttb(raw, 3);
  let tuples: Vec<(f64, f64)> = downsampled.iter().map(|dp| (dp.x, dp.y)).collect();


  println!("{:?}", tuples);
}