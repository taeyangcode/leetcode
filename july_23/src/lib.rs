#![allow(dead_code)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_11;

#[cfg(test)]
mod test_day_08 {
    use crate::day_08::Solution;

    #[test]
    fn test_01() {
        Solution::put_marbles(vec![1, 2, 3, 4, 5, 6, 7], 3);
    }
}
