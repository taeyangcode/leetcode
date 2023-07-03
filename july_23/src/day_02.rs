// https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut cache: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        for request in &requests {
            *cache
                .entry(request[0])
                .or_insert(HashMap::new())
                .entry(request[1])
                .or_insert(0) += 1;
        }

        let mut result: i32 = 0;
        for request in requests {
            result += Self::transfer_request(&mut cache, request[0], request[1], request[0], -1);
        }
        return result;
    }

    fn transfer_request(cache: &mut HashMap<i32, HashMap<i32, i32>>, start_building: i32, target_building: i32, current_building: i32, previous_building: i32) -> i32 {

        return -1;
    }
}
