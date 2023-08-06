// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// https://leetcode.com/submissions/detail/1013348914/

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let lookup: Vec<&'static str> = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        const OFFSET: usize = 2;
        let digits: Vec<&'static str> = digits
            .chars()
            .map(|letter: char| lookup[letter.to_digit(10).unwrap() as usize - OFFSET])
            .collect::<Vec<&'static str>>();
        let mut result: HashSet<String> = HashSet::new();
        let length: usize = digits.len();

        fn find_letter_combinations(digits: &Vec<&'static str>, combinations: &mut HashSet<String>, digit_index: usize, length: &usize, current_combination: String) {
            if digit_index == *length {
                combinations.insert(current_combination);
                return;
            }

            for letter in digits[digit_index].chars() {
                let mut new_combination: String = current_combination.clone();
                new_combination.push(letter);
                find_letter_combinations(digits, combinations, digit_index + 1, length, new_combination);
            }
        }
        find_letter_combinations(&digits, &mut result, 0, &length, String::new());

        return result.into_iter().collect::<Vec<String>>();
    }
}
