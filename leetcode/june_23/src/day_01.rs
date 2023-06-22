// https://leetcode.com/problems/shortest-path-in-binary-matrix/
// https://leetcode.com/problems/shortest-path-in-binary-matrix/submissions/976880911/

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let length: usize = grid.len();
        if grid[0][0] == 1 || grid[length - 1][length - 1] == 1 {
            return -1;
        }
        grid[0][0] = -1;
        return match Self::find_shortest_path(&mut grid, &mut VecDeque::from([(0, 0)]), length, 0, 0) {
            0 => -1,
            result => -1 * result,
        };
    }

    fn find_shortest_path(grid: &mut Vec<Vec<i32>>, queue: &mut VecDeque<(usize, usize)>, length: usize, row: usize, column: usize) -> i32 {
        let count: i32 = grid[row][column];

        if (row, column) == (length - 1, length - 1) {
            return grid[row][column];
        }

        let can_move_north: bool = row > 0;
        let can_move_south: bool = row < length - 1;
        let can_move_west: bool = column > 0;
        let can_move_east: bool = column < length - 1;
        let can_move_north_west: bool = can_move_north && can_move_west;
        let can_move_north_east: bool = can_move_north && can_move_east;
        let can_move_south_west: bool = can_move_south && can_move_west;
        let can_move_south_east: bool = can_move_south && can_move_east;

        let can_visit = |grid: &Vec<Vec<i32>>, new_row: usize, new_column: usize, count: i32| -> bool {
            let visit_value: i32 = grid[new_row][new_column];
            return visit_value == 0 || (visit_value.abs() != 1 && count - 1 > visit_value);
        };

        if can_move_north_east && can_visit(&grid, row - 1, column + 1, count) {
            grid[row - 1][column + 1] += count - 1;
            queue.push_back((row - 1, column + 1));
        }
        if can_move_south_east && can_visit(&grid, row + 1, column + 1, count) {
            grid[row + 1][column + 1] += count - 1;
            queue.push_back((row + 1, column + 1));
        }
        if can_move_south_west && can_visit(&grid, row + 1, column - 1, count) {
            grid[row + 1][column - 1] += count - 1;
            queue.push_back((row + 1, column - 1));
        }
        if can_move_north_west && can_visit(&grid, row - 1, column - 1, count) {
            grid[row - 1][column - 1] += count - 1;
            queue.push_back((row - 1, column - 1));
        }
        if can_move_north && can_visit(&grid, row - 1, column, count) {
            grid[row - 1][column] += count - 1;
            queue.push_back((row - 1, column));
        }
        if can_move_east && can_visit(&grid, row, column + 1, count) {
            grid[row][column + 1] += count - 1;
            queue.push_back((row, column + 1));
        }
        if can_move_south && can_visit(&grid, row + 1, column, count) {
            grid[row + 1][column] += count - 1;
            queue.push_back((row + 1, column));
        }
        if can_move_west && can_visit(&grid, row, column - 1, count) {
            grid[row][column - 1] += count - 1;
            queue.push_back((row, column - 1));
        }

        queue.pop_front();
        if queue.len() == 0 {
            return 1;
        }
        let (new_row, new_column): (usize, usize) = queue[0];
        return Self::find_shortest_path(grid, queue, length, new_row, new_column);
    }
}

/*
 * DFS Solution - Fails on Time Limit Exceeded
impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let length: usize = grid.len();
        if grid[0][0] == 1 || grid[length - 1][length - 1] == 1 {
            return -1;
        }
        Self::find_shortest_path(&mut grid, length, 0, 0, 0);
        let result: i32 = grid[length - 1][length - 1];
        return match result {
            0 => -1,
            result => -1 * result,
        };
    }

    fn find_shortest_path(grid: &mut Vec<Vec<i32>>, length: usize, row: usize, column: usize, count: i32) -> () {
        if (row, column) == (length - 1, length - 1) {
            grid[row][column] = count - 1;
            return;
        }

        let can_move_north: bool = row > 0;
        let can_move_south: bool = row < length - 1;
        let can_move_west: bool = column > 0;
        let can_move_east: bool = column < length - 1;
        let can_move_north_west: bool = can_move_north && can_move_west;
        let can_move_north_east: bool = can_move_north && can_move_east;
        let can_move_south_west: bool = can_move_south && can_move_west;
        let can_move_south_east: bool = can_move_south && can_move_east;

        let mut make_valid_move = |new_row: usize, new_column: usize, current_count: i32| -> () {
            let move_value: i32 = grid[new_row][new_column];
            if move_value == 0 || (move_value != 1 && current_count - 1 > move_value) {
                grid[new_row][new_column] = current_count - 1;
                Self::find_shortest_path(grid, length, new_row, new_column, current_count - 1);
            }
        };

        if can_move_south_east {
            make_valid_move(row + 1, column + 1, count);
        }
        if can_move_south {
            make_valid_move(row + 1, column, count);
        }
        if can_move_east {
            make_valid_move(row, column + 1, count);
        }
        if can_move_south_west {
            make_valid_move(row + 1, column - 1, count);
        }
        if can_move_north_east {
            make_valid_move(row - 1, column + 1, count);
        }
        if can_move_west {
            make_valid_move(row, column - 1, count);
        }
        if can_move_north {
            make_valid_move(row - 1, column, count);
        }
        if can_move_north_west {
            make_valid_move(row - 1, column - 1, count);
        }
    }
}
*/
