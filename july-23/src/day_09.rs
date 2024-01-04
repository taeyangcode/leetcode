// https://leetcode.com/problems/substring-with-largest-variance/
// https://leetcode.com/submissions/detail/1008722662/

pub struct Solution;

enum Offset {
    None,
    O,
    Other,
}

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let characters: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut unique_characters: Vec<char> = characters.clone();
        unique_characters.sort();
        unique_characters.dedup();
        let unique_characters_length: usize = unique_characters.len();

        let mut max_variance: i32 = 0;

        for alpha_index in 0..unique_characters_length {
            for beta_index in alpha_index + 1..unique_characters_length {
                let mut alpha_variance: i32 = 0;
                let mut alpha_offset: Offset = Offset::None;
                let mut beta_variance: i32 = 0;
                let mut beta_offset: Offset = Offset::None;
                for current_letter in &characters {
                    match *current_letter {
                        _ if *current_letter == unique_characters[alpha_index] => {
                            alpha_variance += 1;
                            match alpha_offset {
                                Offset::O => max_variance = std::cmp::max(max_variance, alpha_variance),
                                Offset::Other => max_variance = std::cmp::max(max_variance, alpha_variance - 1),
                                _ => {},
                            };

                            beta_offset = Offset::O;
                            beta_variance -= 1;
                            if beta_variance < 0 {
                                beta_offset = Offset::Other;
                                beta_variance = 0;
                            }
                        },
                        _ if *current_letter == unique_characters[beta_index] => {
                            beta_variance += 1;
                            match beta_offset {
                                Offset::O => max_variance = std::cmp::max(max_variance, beta_variance),
                                Offset::Other => max_variance = std::cmp::max(max_variance, beta_variance - 1),
                                _ => {},
                            };

                            alpha_offset = Offset::O;
                            alpha_variance -= 1;
                            if alpha_variance < 0 {
                                alpha_offset = Offset::Other;
                                alpha_variance = 0;
                            }
                        },
                        _ => {
                            match alpha_offset {
                                Offset::O => max_variance = std::cmp::max(max_variance, alpha_variance),
                                Offset::Other => max_variance = std::cmp::max(max_variance, alpha_variance - 1),
                                _ => alpha_offset = Offset::Other,
                            };
                            match beta_offset {
                                Offset::O => max_variance = std::cmp::max(max_variance, beta_variance),
                                Offset::Other => max_variance = std::cmp::max(max_variance, beta_variance - 1),
                                _ => beta_offset = Offset::Other,
                            };
                        },
                    };
                    match alpha_offset {
                        Offset::O => max_variance = std::cmp::max(max_variance, alpha_variance),
                        Offset::Other => max_variance = std::cmp::max(max_variance, alpha_variance - 1),
                        _ => {},
                    };
                    match beta_offset {
                        Offset::O => max_variance = std::cmp::max(max_variance, beta_variance),
                        Offset::Other => max_variance = std::cmp::max(max_variance, beta_variance - 1),
                        _ => {},
                    };
                }
            }
        }

        return max_variance;
    }
}
