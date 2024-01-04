// https://leetcode.com/problems/time-needed-to-inform-all-employees/
// https://leetcode.com/problems/time-needed-to-inform-all-employees/submissions/981408983/

pub struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut tree: Vec<Vec<[i32; 2]>> = vec![vec![]; n as usize];
        for index in 0..n as usize {
            match index {
                _ if index == head_id as usize => continue,
                _ if manager[index] == -1 => continue,
                index => tree[manager[index] as usize].push([index as i32, -1]),
            };
        }
        return Self::find_time_needed(&mut tree, &manager, &inform_time, head_id as usize);
    }

    fn find_time_needed(tree: &mut Vec<Vec<[i32; 2]>>, manager: &Vec<i32>, inform_time: &Vec<i32>, index: usize) -> i32 {
        if tree[index].len() == 0 {
            return inform_time[index];
        }
        let mut max = -1;
        for node_index in 0..tree[index].len() {
            if tree[index][node_index][1] == -1 {
                tree[index][node_index][1] = Self::find_time_needed(tree, manager, inform_time, tree[index][node_index][0] as usize);
            }
            max = std::cmp::max(max, tree[index][node_index][1]);
        }
        return max + inform_time[index];
    }
}
