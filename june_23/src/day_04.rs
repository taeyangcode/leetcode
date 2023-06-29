// https://leetcode.com/problems/number-of-provinces/
// https://leetcode.com/problems/number-of-provinces/submissions/982195761/

pub struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;
        let length: usize = is_connected.len();
        let mut cache: Vec<bool> = vec![false; length];

        for province_index in 0..length {
            result += Self::find_provinces(&is_connected, province_index, &mut cache);
        }

        return result;
    }

    fn find_provinces(is_connected: &Vec<Vec<i32>>, province_index: usize, cache: &mut Vec<bool>) -> i32 {
        let mut is_province: bool = true;
        let mut queue: Vec<usize> = Vec::new();
        for city_index in 0..is_connected[province_index].len() {
            if city_index == province_index {
                cache[city_index] = true;
                continue;
            }

            if is_connected[province_index][city_index] == 1 {
                match cache[city_index] {
                    true => is_province = false,
                    false => {
                        cache[city_index] = true;
                        queue.push(city_index);
                    },
                };
            }
        }
        for index in queue {
            let _ = Self::find_provinces(is_connected, index, cache);
        }
        return is_province as i32;
    }
}
