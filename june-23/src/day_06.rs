// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/
// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/submissions/982524244/

pub struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let difference: i32 = arr[1] - arr[0];
        for index in 1..arr.len() - 1 {
            if arr[index + 1] - arr[index] != difference {
                return false;
            }
        }
        return true;
    }
}
