// https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests/

pub struct Solution;

use std::{collections::HashMap, ops::IndexMut};

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut cache: HashMap<i32, Vec<i32>> = HashMap::new();
        for request in &requests {
            *cache
                .entry(request[0])
                .or_insert(vec![0; n as usize])
                .index_mut(request[1] as usize) += 1;
        }
        println!("{:?}", cache);

        let mut result: i32 = 0;
        for request in requests {
            if request[0] == request[1] {
                result += 1;
                cache.get_mut(&request[0]).unwrap()[request[0] as usize] -= 1;
                continue;
            }
            result += Self::transfer_request(&mut cache, request[0], request[0], request[1]);
            println!("result: {result}");
        }
        return result;
    }

    fn transfer_request(cache: &mut HashMap<i32, Vec<i32>>, start_building: i32, current_building: i32, target: i32) -> i32 {
        if !cache.contains_key(&current_building) {
            println!("doesn't contain! {current_building}");
            return -1;
        }

        let (mut max, mut max_index): (i32, i32) = (-1, -1);
        let target_buildings: Vec<(usize, i32)> = cache[&current_building]
            .clone()
            .into_iter()
            .enumerate()
            .filter(|&(_, target)| target > 0)
            .collect::<Vec<(usize, i32)>>();
        println!("{:?}", target_buildings);

        for (target_building, _) in target_buildings {
            cache.get_mut(&current_building).unwrap()[target_building as usize] -= 1;
            match Self::transfer_request(cache, start_building, target_building as i32, target) {
                result if result > max => {
                    if max_index != -1 {
                        cache.get_mut(&current_building).unwrap()[max_index as usize] += 1;
                    }
                    max = result;
                    max_index = target_building as i32;
                },
                _ => cache.get_mut(&current_building).unwrap()[target_building as usize] += 1,
            };
        }
        println!("{max}");

        return std::cmp::max(max + 1, (current_building == target) as i32);
    }
}
