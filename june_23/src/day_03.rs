// https://leetcode.com/problems/time-needed-to-inform-all-employees/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut cache: HashMap<i32, i32> = HashMap::new();
        cache.insert(head_id, 0);

        for index in 0..n as usize {
        }

        return result;
    }

    fn find_num_of_minutes(n: &usize, head_id: &usize, manager: &Vec<i32>, inform_time: &Vec<i32>, cache: &mut HashMap<i32, i32>, index: usize) -> () {
        match cache.get(&manager[index]) {
        };
    }
}
