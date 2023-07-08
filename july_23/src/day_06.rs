// https://leetcode.com/problems/minimum-size-subarray-sum/

pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let length: usize = nums.len();
        let mut minimum_length = std::i32::MAX;
        let mut subarray: Vec<i32> = vec![];
        let mut current_sum: i32 = 0;

        for index in 0..length {
            Self::find_minimum_size(&target, &nums, &mut subarray, &mut minimum_length, &length, &mut current_sum, index);
        }

        match subarray.into_iter().sum::<i32>() < target {
            true => return 0,
            false => minimum_length,
        }
    }

    fn find_minimum_size(target: &i32, nums: &Vec<i32>, subarray: &mut Vec<i32>, minimum_length: &mut i32, length: &usize, current_sum: &mut i32, current_index: usize) -> () {
        subarray.push(nums[current_index]);
        *current_sum += nums[current_index];
        if *current_sum < *target {
            return;
        }

        loop {
            let mut index: usize = current_index;
            let first: i32 = subarray[0];
            let last: i32 = nums[index];
            match std::cmp::min(first, last) {
                result if result == first => {
                    if *current_sum - first < *target {
                        return;
                    }
                    *current_sum -= first;
                    subarray.remove(0);
                    index -= 1;
                    *minimum_length = std::cmp::min(*minimum_length, subarray.len() as i32 - 1);
                    println!("subarray: {:?}, minimum: {minimum_length}, length: {}, sum: {current_sum}", subarray, subarray.len());
                },
                result if result == last => {
                    if *current_sum - last < *target {
                        return;
                    }
                    *minimum_length = std::cmp::min(*minimum_length, subarray.len() as i32 - 1);
                    *subarray = vec![];
                    *current_sum = 0;
                    println!("subarray: {:?}, minimum: {minimum_length}, length: {}, sum: {current_sum}", subarray, subarray.len());
                    return;
                },
                _ => return,
            };
        }
    }
}
