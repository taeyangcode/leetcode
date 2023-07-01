// https://leetcode.com/problems/fair-distribution-of-cookies/
// https://leetcode.com/problems/fair-distribution-of-cookies/submissions/983453115/

pub struct Solution;

impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let length: usize = cookies.len();

        let mut distributions: Vec<i32> = vec![];
        let mut start: Vec<i32> = vec![0; k as usize];
        start[0] = cookies[0];

        Self::find_maximum_unfairness(&mut distributions, start, &cookies, 1, length, k as usize);
        return distributions.into_iter().min().unwrap();
    }

    fn find_maximum_unfairness(distributions: &mut Vec<i32>, distribution: Vec<i32>, cookies: &Vec<i32>, index: usize, length: usize, children: usize) -> () {
        if index == length {
            distributions.push(*distribution.iter().max().unwrap());
            return;
        }

        for cookie_bag in 0..children {
            let mut new_distribution: Vec<i32> = distribution.clone();
            new_distribution[cookie_bag] += cookies[index];
            Self::find_maximum_unfairness(distributions, new_distribution, cookies, index + 1, length, children);
        }
    }
}
