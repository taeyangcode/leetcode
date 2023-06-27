// https://leetcode.com/problems/time-needed-to-inform-all-employees/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut cache: HashMap<i32, i32> = HashMap::new();
        for index in 0..n {
            let manager_index: &i32 = &manager[index as usize];
            if cache.get(manager_index).is_none() {
                cache.insert(*manager_index, 1);
                result += inform_time[*manager_index as usize];
            }
        }

        return result;
    }
}
