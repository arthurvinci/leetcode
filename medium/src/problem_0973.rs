use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Solution;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(coords: &[i32]) -> Self {
        Self {
            x: coords[0],
            y: coords[1],
        }
    }

    fn dist_from_origin(&self) -> f64 {
        (self.x.pow(2) as f64 + self.y.pow(2) as f64).sqrt()
    }

    fn into_vec(self) -> Vec<i32> {
        vec![self.x, self.y]
    }
}

impl Eq for Point {}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist_from_origin().total_cmp(&other.dist_from_origin())
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let points: Vec<Point> = points.iter().map(|point| Point::from(point)).collect();
        let mut heap = BinaryHeap::from(points);

        while heap.len() > k as usize {
            heap.pop();
        }

        heap.into_iter().map(|point| point.into_vec()).collect()
    }
}
