// https://leetcode.com/problems/minimum-size-subarray-sum/

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut length: usize = nums.len();
        let mut subarray: VecDeque<i32> = VecDeque::new();
        let mut sum: i32 = 0;

        Self::find_subarray(target, &nums, &length, &mut subarray, &mut sum, 0);

        return subarray.len() as i32;
    }

    fn find_subarray(target: i32, nums: &Vec<i32>, length: &usize, subarray: &mut VecDeque<i32>, sum: &mut i32, index: usize) {
        if index >= *length {
            return;
        }

        *sum += nums[index];
        subarray.push_back(nums[index]);

        if *sum <= target {
            Self::find_subarray(target, nums, length, subarray, sum, index + 1);
            return;
        }

        let mut last_index: usize = index;
        loop {
            println!("sum: {sum}, subarray: {:?}, index: {index}", subarray);
            match std::cmp::min(subarray[0], nums[last_index]) {
                first if first == subarray[0] => {
                    println!("B");
                    if *sum - first < target {
                        println!("A");
                        Self::find_subarray(target, nums, length, subarray, sum, index + 1);
                        return;
                    }
                    *sum -= first;
                    subarray.pop_front();
                },
                last => {
                    if *sum - last < target {
                        return;
                    }
                },
            };
        }
    }
}
