// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/
// https://leetcode.com/submissions/detail/987340024/

pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let length: i32 = nums.len() as i32;
        let (mut max, mut zero_index, mut streak): (i32, i32, i32) = (0, -1, 0);
        for current_index in 0..length {
            match nums[current_index as usize] {
                0 => {
                    match zero_index {
                        -1 => zero_index = current_index,
                        _ => {
                            streak = current_index - zero_index - 1;
                            zero_index = current_index;
                        },
                    };
                },
                1 => streak += 1,
                _ => {},
            };
            if streak > max {
                max = streak;
            }
        }
        if zero_index == length - 1 && max == 0 {
            return 0;
        }
        if zero_index == -1 {
            return length as i32 - 1;
        }
        return max;
    }
}
