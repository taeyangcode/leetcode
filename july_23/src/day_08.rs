// https://leetcode.com/problems/put-marbles-in-bags/

pub struct Solution;

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let weights_length: usize = weights.len();
        let mut bags: Vec<Vec<i32>> = vec![vec![]; k as usize];

        let mut bag_index: usize = 0;
        let mut weight_index: usize = 0;
        while weight_index < weights_length {
            bags[bag_index].push(weights[weight_index]);
            weight_index += 1;
            bag_index += (weight_index > weights_length - k as usize) as usize;
        }

        return -1;
    }
}
