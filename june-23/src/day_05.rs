// https://leetcode.com/problems/check-if-it-is-a-straight-line/
// https://leetcode.com/problems/check-if-it-is-a-straight-line/submissions/982220403/

pub struct Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let slope: f32 = match (coordinates[1][0] - coordinates[0][0]) as f32 {
            denominator if denominator as i32 == 0 => {
                for index in 2..coordinates.len() {
                    if coordinates[index][0] != coordinates[0][0] {
                        return false;
                    }
                }
                return true;
            },
            denominator => (coordinates[1][1] - coordinates[0][1]) as f32 / (denominator) as f32,
        };
        let y_intercept: f32 = coordinates[0][1] as f32 - slope * coordinates[0][0] as f32;
        for index in 2..coordinates.len() {
            if coordinates[index][1] as f32 != slope * coordinates[index][0] as f32 + y_intercept {
                return false;
            }
        }
        return true;
    }
}
