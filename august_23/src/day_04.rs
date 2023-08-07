// https://leetcode.com/problems/word-break/

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_dict: HashSet<String> = word_dict.into_iter().collect::<HashSet<String>>();

        fn segmentable(string: &String, word_dict: &HashSet<String>, string_length: &usize, left_index: usize, right_index: usize) -> bool {
            if left_index >= *string_length {
                return true;
            }
            if right_index <= left_index {
                return false;
            }

            for new_right_index in (left_index..right_index).rev() {
                println!("{}", &string[left_index..new_right_index]);
                if word_dict.contains(&string[left_index..new_right_index]) {
                    if segmentable(string, word_dict, string_length, new_right_index, *string_length + 1) == true {
                        return true;
                    }
                }
            }

            return false;
        }

        return segmentable(&s, &word_dict, &s.len(), 0, s.len() + 1);
    }
}
