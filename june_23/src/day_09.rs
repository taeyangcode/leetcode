// https://leetcode.com/problems/find-smallest-letter-greater-than-target/
// https://leetcode.com/problems/find-smallest-letter-greater-than-target/submissions/982756991/

pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        match target {
            'z' | _ if target >= letters[letters.len() - 1] => return letters[0],
            _ => {},
        };
        let mut current_max: char = target;
        for letter in letters {
            if letter > target && letter > current_max {
                return letter;
            }
            current_max = letter;
        }
        return current_max;
    }
}
