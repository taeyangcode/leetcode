// https://leetcode.com/problems/kth-largest-element-in-an-array/

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let length: usize = nums.len();

        let mut sorted_nums: Vec<i32> = vec![];
        let (mut minimum, mut maximum): (usize, usize) = (0, 0);
        for number in nums {
            let sorted_length: usize = sorted_nums.len();

            match number.signum() {
                1 => {
                    if number as usize > maximum {
                        maximum = number as usize;
                        sorted_nums.resize(maximum + minimum, 0);
                    }
                },
                -1 => {
                    let absolute_number: usize = number.abs() as usize;
                    if absolute_number > sorted_length - maximum {
                        minimum = absolute_number;
                        sorted_nums.resize(maximum + minimum, 0);
                    }
                },
                0 => {
                    if sorted_nums.len() == 0 {
                        sorted_nums.push(0);
                    }
                },
                _ => {},
            };

            let sorted_length: usize = sorted_nums.len();
            match number.signum() {
                1 => sorted_nums[number as usize + minimum - 1] += 1,
                -1 => sorted_nums[sorted_length - maximum - number as usize - 1] += 1,
                0 => sorted_nums[0] += 1,
                _ => {},
            };
        }

        panic!();
    }
}
