#![allow(dead_code)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_11;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        crate::day_09::Solution::largest_variance(String::from("icexiahccknibwuwgi"));
    }
}
