use january_24::day_04::Solution;

#[test]
fn cases() {
    assert_eq!(Solution::min_operations(Vec::from([2,3,3,2,2,4,2,3,4])), 4);
    assert_eq!(Solution::min_operations(Vec::from([2,1,2,2,3,3])), -1);
}
