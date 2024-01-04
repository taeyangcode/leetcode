// question: https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/description/?envType=daily-question&envId=2024-01-04
// solution: https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/submissions/1136332036/?envType=daily-question&envId=2024-01-04

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        return nums
            .into_iter()
            .fold(HashMap::<_, i32>::new(), |mut accumulator, value| {
                *accumulator.entry(value).or_default() += 1;
                accumulator
            })
            .into_iter()
            .try_fold(0, |operations, (_, value_amount)| {
                match value_amount {
                    1 => Err(()),
                    _ => Ok(operations + (value_amount as f32 / 3.0).ceil() as i32),
                }
            })
            .unwrap_or(-1)
    }
}
