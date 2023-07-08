// https://leetcode.com/problems/minimum-size-subarray-sum/

pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut length: usize = nums.len();
        let mut subarray: Vec<i32> = vec![];
        let mut sum: i32 = 0;

        return Self::find_subarray(target, &nums, &length, &mut subarray, &mut sum, 0);
    }

    fn find_subarray(target: i32, nums: &Vec<i32>, length: &usize, subarray: &mut Vec<i32>, sum: &mut i32, index: usize) -> i32 {
        if index >= length {
            return 0;
        }

        *sum += nums[index];
        subarray.push(nums[index]);

        if *sum <= target {
            return 1 + Self::find_subarray(target, nums, length, subarray, sum, index);
        }

        while *sum - subarray[0] >= target {
            *sum -= subarray[0];
            subarray.remove(0);
        }
        let mut left_sum: i32 = subarray[0];
        let mut right_sum: i32 = nums[index];
        let mut current_index: usize = 1;
        while left_sum < right_sum && index + current_index < length {
            left_sum += subarray[current_index];
            right_sum += nums[index + current_index];
            current_index += 1;
        }
        if left_sum < right_sum {
            return;
        }
    }
}
