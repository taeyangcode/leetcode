// https://leetcode.com/problems/detonate-the-maximum-bombs/
// https://leetcode.com/problems/detonate-the-maximum-bombs/submissions/978887839/

pub struct Solution;

use std::collections::HashSet;

type Circle = (i32, i32, i32);
impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let length: usize = bombs.len();
        let mut cache: Vec<HashSet<usize>> = vec![HashSet::new(); length];
        for index in 0..length {
            Self::find_maximum_detonation(&bombs, &mut cache, length, index);
        }
        return cache.into_iter().max_by(|current, next| current.len().cmp(&next.len())).unwrap().len() as i32;
    }

    fn find_maximum_detonation(bombs: &Vec<Vec<i32>>, cache: &mut Vec<HashSet<usize>>, length: usize, index: usize) -> () {
        let bomb: Circle = (bombs[index][0], bombs[index][1], bombs[index][2]);
        for current_index in 0..length {
            if cache[index].contains(&current_index) {
                let clone: HashSet<usize> = cache[current_index].clone();
                cache[index].extend(clone);
                continue;
            }
            if current_index == index {
                cache[index].insert(index);
                continue;
            }
            let current_bomb: Circle = (bombs[current_index][0], bombs[current_index][1], bombs[current_index][2]);
            if Self::contains_circle(bomb, current_bomb) {
                if cache[current_index].is_empty() {
                    cache[index].insert(current_index);
                    Self::find_maximum_detonation(bombs, cache, length, current_index);
                }
                let clone: HashSet<usize> = cache[current_index].clone();
                cache[index].extend(clone);
            }
        }
    }

    fn contains_circle(circle_one: Circle, circle_two: Circle) -> bool {
        return circle_one.2 as f64 >= (((circle_two.1 - circle_one.1) as f64).powf(2.0) + ((circle_two.0 - circle_one.0) as f64).powf(2.0)).sqrt();
   }
