// https://leetcode.com/problems/word-break/
// https://leetcode.com/submissions/detail/1018182079/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        if word_dict.contains(&s) {
            return true;
        }

        let mut dictionary: HashMap<&str, bool> = word_dict
            .iter()
            .map(|string: &String| (string.as_str(), true))
            .collect::<HashMap<&str, bool>>();
        dictionary.insert(&"", true);

        fn is_segmentable<'a>(string: &'a str, dictionary: &mut HashMap<&'a str, bool>, word_length: usize, left_index: usize, right_index: usize) -> bool {
            for index in left_index + 1..right_index {
                let left_string: &str = &string[left_index..index];
                let right_string: &str = &string[index..right_index];

                let left_result: Option<bool> = dictionary.get(left_string).copied();
                let right_result: Option<bool> = dictionary.get(right_string).copied();
                match (left_result, right_result) {
                    (Some(true), Some(true)) => return true,
                    (Some(_), Some(_)) => {},
                    (Some(left_segmentable), None) => {
                        let right_segmentable: bool = is_segmentable(string, dictionary, word_length, index, right_index);
                        dictionary.insert(right_string, right_segmentable);
                        if left_segmentable && right_segmentable {
                            return true;
                        }
                    },
                    (None, Some(right_segmentable)) => {
                        let left_segmentable: bool = is_segmentable(string, dictionary, word_length, left_index, index);
                        dictionary.insert(left_string, left_segmentable);
                        if left_segmentable && right_segmentable {
                            return true;
                        }
                    },
                    (None, None) => {
                        let left_segmentable: bool = is_segmentable(string, dictionary, word_length, left_index, index);
                        let right_segmentable: bool = is_segmentable(string, dictionary, word_length, index, right_index);

                        dictionary.insert(left_string, left_segmentable);
                        dictionary.insert(right_string, right_segmentable);

                        if left_segmentable && right_segmentable {
                            return true;
                        }
                    },
                };
            }

            return false;
        }

        return is_segmentable(&s, &mut dictionary, s.len(), 0, s.len());
    }
}
