// https://leetcode.com/problems/buddy-strings/
// https://leetcode.com/submissions/detail/985925877/

pub struct Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let mut lowercase_buffer: [u8; 27] = [0; 27];
        let (mut result, (mut search_0, mut search_1)): (u8, (u8, u8)) = (0, (0, 0));
        let (string, goal): (&[u8], &[u8]) = (s.as_bytes(), goal.as_bytes());
        for index in 0..s.len() {
            lowercase_buffer[(string[index] - 97) as usize] += 1;
            lowercase_buffer[26] += (lowercase_buffer[(string[index] - 97) as usize] > 1) as u8;
            if string[index] != goal[index] {
                result = match result {
                    0 => {
                        search_0 = goal[index];
                        search_1 = string[index];
                        1
                    },
                    1 => {
                        if string[index] != search_0 || goal[index] != search_1 {
                            return false;
                        }
                        2
                    },
                    _ => {
                        return false;
                    }
                };
            }
        }
        return result == 2 || (result == 0 && lowercase_buffer[26] > 0);
    }
}
