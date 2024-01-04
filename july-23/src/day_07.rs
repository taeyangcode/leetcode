// https://leetcode.com/problems/maximize-the-confusion-of-an-exam/
// https://leetcode.com/submissions/detail/990574732/

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let length: usize = answer_key.len();
        let answer_key: Vec<u8> = answer_key.into_bytes();
        let mut max_consecutive: i32 = std::i32::MIN;

        let mut true_switch: VecDeque<usize> = VecDeque::new();
        let mut false_switch: VecDeque<usize> = VecDeque::new();
        let mut true_consecutive: i32 = 0;
        let mut false_consecutive: i32 = 0;

        for index in 0..length {
            let true_length: usize = true_switch.len();
            let false_length: usize = false_switch.len();

            let current_letter: char = answer_key[index] as char;
            match current_letter {
                'T' => {
                    true_consecutive += 1;
                    max_consecutive = std::cmp::max(max_consecutive, true_consecutive);

                    match false_length < k as usize {
                        true => {
                            false_consecutive += 1;
                            false_switch.push_back(index);
                            max_consecutive = std::cmp::max(max_consecutive, false_consecutive);
                        },
                        false => {
                            max_consecutive = std::cmp::max(max_consecutive, false_consecutive);
                            false_consecutive = index as i32 - false_switch[0] as i32;
                            false_switch.pop_front();
                            false_switch.push_back(index);
                        },
                    };
                },
                'F' => {
                    false_consecutive += 1;
                    max_consecutive = std::cmp::max(max_consecutive, false_consecutive);

                    match true_length < k as usize {
                        true => {
                            true_consecutive += 1;
                            true_switch.push_back(index);
                            max_consecutive = std::cmp::max(max_consecutive, true_consecutive);
                        },
                        false => {
                            max_consecutive = std::cmp::max(max_consecutive, true_consecutive);
                            true_consecutive = index as i32 - true_switch[0] as i32;
                            true_switch.pop_front();
                            true_switch.push_back(index);
                        },
                    };
                },
                _ => {},
            };
        }

        return max_consecutive;
    }
}
