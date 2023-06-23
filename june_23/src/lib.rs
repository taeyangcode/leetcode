#![allow(clippy::needless_return)]

mod day_01;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_01_00() -> () {
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        matrix.push(vec![0, 1]);
        matrix.push(vec![1, 0]);
        assert!(day_01::Solution::shortest_path_binary_matrix(matrix) == 2);
    }

    #[test]
    fn day_01_01() -> () {
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        matrix.push(vec![0, 0, 0]);
        matrix.push(vec![1, 1, 0]);
        matrix.push(vec![1, 1, 0]);
        assert!(day_01::Solution::shortest_path_binary_matrix(matrix) == 4);
    }

    #[test]
    fn day_01_02() -> () {
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        matrix.push(vec![0, 0, 0]);
        matrix.push(vec![0, 1, 0]);
        matrix.push(vec![0, 0, 0]);
        assert!(day_01::Solution::shortest_path_binary_matrix(matrix) == 4);
    }

    #[test]
    fn day_01_03() -> () {
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        matrix.push(vec![0,1,1,0,0,0]);
        matrix.push(vec![0,1,0,1,1,0]);
        matrix.push(vec![0,1,1,0,1,0]);
        matrix.push(vec![0,0,0,1,1,0]);
        matrix.push(vec![1,1,1,1,1,0]);
        matrix.push(vec![1,1,1,1,1,0]);
        assert!(day_01::Solution::shortest_path_binary_matrix(matrix) == 14);
    }
}
