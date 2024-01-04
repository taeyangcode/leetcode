// https://leetcode.com/problems/minimum-size-subarray-sum/
// https://leetcode.com/submissions/detail/990536561/

pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left_index: usize = 0;
        let mut right_index: usize = 0;
        let mut sum: i32 = 0;
        let mut minimum_length: usize = std::usize::MAX;

        for number in &nums {
            sum += number;
            if sum >= target {
                while sum >= target {
                    minimum_length = std::cmp::min(minimum_length, right_index - left_index + 1);
                    sum -= nums[left_index];
                    left_index += 1;
                }
            }
            right_index += 1;
        }

        return (minimum_length != std::usize::MAX) as i32 * minimum_length as i32;
    }
}
