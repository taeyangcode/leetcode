// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/

pub struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        return grid.into_iter().flatten().collect::<Vec<i32>>()
            .into_iter().fold(0, |accumulator: i32, element: i32| accumulator + ((element < 0) as i32));
    }
}
