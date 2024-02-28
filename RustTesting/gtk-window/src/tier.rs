use crate::bin::Bin;
use std::mem;
use std::sync::{Arc, RwLock};

pub struct TierData {
    pub x_stats: Vec<Bin>,
    pub y_stats: Vec<Bin>,
    pub condition: usize,
}

impl TierData {
    pub fn new(condition: usize) -> Self {
        Self { 
            x_stats: vec![Bin::default()],
            y_stats: vec![Bin::default()],
            condition: condition,
        }
    }
}