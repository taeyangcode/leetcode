// https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/
// https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/submissions/982658000/

pub struct Solution;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut result: i32 = 0;
        let length: i32 = (([a, b, c].into_iter().max().unwrap() as f32).log2() + 1.0) as i32;
        for index in 0..length {
            result += match ((a & (1 << index)) != 0, (b & (1 << index)) != 0, (c & (1 << index)) != 0) {
                (true, true, false) => 2,
                (false, false, true) | (false, true, false) | (true, false, false) => 1,
                _ => 0,
            };
        }
        return result;
    }
}
