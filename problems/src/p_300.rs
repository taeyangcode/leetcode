// question: https://leetcode.com/problems/longest-increasing-subsequence/description/
// solution:

pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut length_cache = vec![1; nums.len()];

        return nums
            .iter()
            .enumerate()
            .rev()
            .fold(1_i32, |longest_length, (index, number)| {
                if let Some(remaining_elements) = nums.get(index + 1..) {
                    length_cache[index] += remaining_elements.iter().enumerate().fold(
                        0_i32,
                        |accumulator, (element_index, element)| match number < element {
                            true => {
                                std::cmp::max(accumulator, length_cache[index + element_index + 1])
                            }
                            false => accumulator,
                        },
                    );
                }
                return std::cmp::max(longest_length, length_cache[index]);
            });
    }
}
