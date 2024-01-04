#![allow(dead_code)]

mod day_04;

#[cfg(test)]
mod test {
    #[test]
    fn day_04() {
        assert_eq!(crate::day_04::Solution::min_operations(Vec::from([2,3,3,2,2,4,2,3,4])), 4);
        assert_eq!(crate::day_04::Solution::min_operations(Vec::from([2,1,2,2,3,3])), -1);
    }
}
