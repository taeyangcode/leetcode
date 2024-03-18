// https://leetcode.com/problems/longest-palindromic-substring/?envType=daily-question&envId=2023-10-27

pub struct Solution;

use std::collections::{HashSet, LinkedList};

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut cache: HashSet<&str> = HashSet::<&str>::new();

        fn is_palindrone<'a>(string: &'a str, cache: &mut HashSet<&'a str>) -> bool {
            if cache.contains(&string) { return false; }

            let length: usize = string.len();
            match length {
                0 | 1 => true,
                _ => {
                    match string[0..1] == string[length - 1..length] {
                        true => is_palindrone(&string[1..length - 1], cache),
                        false => { cache.insert(string); false },
                    }
                },
            }
        };

        let mut stack: LinkedList<&str> = LinkedList::from([s.as_str()]);

        fn find_palindrone<'a>(stack: &mut LinkedList<&'a str>, cache: &mut HashSet<&'a str>) -> String {
            let string: &str = stack.pop_front().unwrap();
            match is_palindrone(string, cache) {
                true => return string.to_string(),
                false => {
                    let length: usize = string.len();
                    let before_string: &str = &string[0..length - 1];
                    if !cache.contains(&before_string) {
                        cache.insert(before_string);
                        stack.push_back(before_string);
                    }
                    let after_string: &str = &string[1..length];
                    if !cache.contains(&after_string) {
                        cache.insert(after_string);
                        stack.push_back(after_string);
                    }
                    return find_palindrone(stack, cache);
                },
            }
        };

        return find_palindrone(&mut stack, &mut cache);
    }
}
