// https://leetcode.com/problems/minimum-size-subarray-sum/

pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut subarray: Vec<i32> = vec![];
        let mut nums_index: usize = 0;
        let mut current_sum: i32 = std::i32::MAX;

        Self::find_minimum_subarray(&target, &nums, &nums.len(), &mut subarray, nums_index, &mut current_sum);

        return subarray.len() as i32;
    }

    fn find_minimum_subarray(target: &i32, nums: &Vec<i32>, length: &usize, subarray: &mut Vec<i32>, nums_index: usize, current_sum: &mut i32) {
        if *current_sum + nums[nums_index] <= *target {
            *current_sum += nums[nums_index];
            subarray.push(nums[nums_index]);
            Self::find_minimum_subarray(target, nums, length, subarray, nums_index + 1, current_sum);
            return;
        }

        let new_sum: i32 = *current_sum + nums[nums_index];
        if new_sum - subarray[0] >= *target {
            *current_sum += nums[nums_index] - subarray[0];
            subarray.remove(0);
            subarray.push(nums[nums_index]);
            Self::find_minimum_subarray(target, nums, length, subarray, nums_index, current_sum);
            return;
        }
    }
}
