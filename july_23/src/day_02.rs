// https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;

        let mut cache: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        for request in &requests {
            *cache
                .entry(request[0])
                .or_insert(HashMap::new())
                .entry(request[1])
                .or_insert(0) += 1;
        }

        return result;
    }

    fn transfer_request(cache: &mut HashMap<i32, HashMap<i32, i32>>, start_building: i32, current_building: i32) -> i32 {
        if let Some(building) = cache.get_mut(&current_building) {
            if let Some(target) = building.get_mut(&start_building) {
                if target > &mut 0 {
                    *target -= 1;
                    return 1;
                }
            }
            for target in building {
                if target.1 > &mut 0 {
                    Self::transfer_request(cache, start_building, current_building);
                }
            }
        }
        return 0;
    }
}
