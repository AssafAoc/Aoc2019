use std::ops::Sub;
use std::ops::Add;
use std::cmp::{max, min};

pub fn manhattan_distance<T: Copy + Ord>(p1: (T, T), p2: (T, T)) -> T {
    max(p1.0, p2.0) - min(p1.0, p2.0) + max(p1.1, p2.1) - min(p1.1, p2.1)
}
