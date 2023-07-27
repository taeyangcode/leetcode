// https://leetcode.com/problems/put-marbles-in-bags/
// https://leetcode.com/problems/put-marbles-in-bags/submissions/1005724232/

pub struct Solution;

impl Solution {
    pub fn put_marbles(mut weights: Vec<i32>, k: i32) -> i64 {
        let length: usize = weights.len();

         weights = weights
            .windows(2)
            .map(|weight_element| weight_element[0] + weight_element[1])
            .collect::<Vec<i32>>();
         weights.sort();

         let (mut min_sum, mut max_sum): (i64, i64) = (0, 0);
         for index in 0..k as usize - 1 {
            min_sum += weights[index] as i64;
            max_sum += weights[length - 1 - index] as i64;
         }
         return max_sum - min_sum;
    }
}

/*
 * Initial Intuition Solution - FAIL
 *
use std::collections::VecDeque;

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let weights_length: usize = weights.len();
        let mut bags: Vec<VecDeque<i32>> = vec![VecDeque::new(); k as usize];

        let mut bag_index: usize = 0;
        let mut weight_index: usize = 0;
        while weight_index < weights_length {
            bags[bag_index].push_back(weights[weight_index]);
            weight_index += 1;
            bag_index += (weight_index > weights_length - k as usize) as usize;
        }

        let mut min: i64 = std::i64::MAX;
        let mut max: i64 = std::i64::MIN;
        Self::find_bags(&mut bags, &mut max, &mut min, &(k as usize), 0);

        return max - min;
    }

    fn find_bags(bags: &mut Vec<VecDeque<i32>>, max: &mut i64, min: &mut i64, bag_limit: &usize, bag_index: usize) -> () {
        let current_score: i64 = bags
            .iter()
            .fold(0, |accumulator: i64, current_bag: &VecDeque<i32>|
                  accumulator + *current_bag.front().unwrap() as i64 + *current_bag.back().unwrap() as i64);
        *max = std::cmp::max(*max, current_score);
        *min = std::cmp::min(*min, current_score);
        println!("{:?}", bags);
        println!("current score: {current_score}, max: {max}, min: {min}");

        let has_one_marble: bool = bags[bag_index].len() == 1;
        let is_last_bag: bool = *bag_limit == bag_index + 1;
        if has_one_marble || is_last_bag {
            return;
        }

        loop {
            if bags[bag_index].len() == 1 {
                let next_element: i32 = bags[bag_index + 1].pop_front().unwrap();
                bags[bag_index].push_back(next_element);
                return;
            }

            let pop_element: i32 = bags[bag_index].pop_back().unwrap();
            bags[bag_index + 1].push_front(pop_element);

            Self::find_bags(bags, max, min, bag_limit, bag_index + 1);
        }
    }
}
*/
