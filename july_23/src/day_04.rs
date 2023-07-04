// https://leetcode.com/problems/single-number-ii/
// https://leetcode.com/problems/single-number-ii/submissions/985945116/

pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::from([(1, HashSet::new()), (2, HashSet::new())]);
        nums
            .into_iter()
            .for_each(|number: i32| {
                if !map[&2].contains(&number) {
                    match map[&1].contains(&number) {
                        true => { map.get_mut(&1).unwrap().remove(&number); map.get_mut(&2).unwrap().insert(number); },
                        false => { map.get_mut(&1).unwrap().insert(number); },
                    };
                }
            });

        return *map[&0].iter().last().unwrap();
    }
}
