// https://leetcode.com/problems/combinations/
// https://leetcode.com/submissions/detail/1012465880/

struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combinations: Vec<Vec<i32>> = vec![];
        combinations.push(
            (1..=k)
                .map(|number| number)
                .collect::<Vec<i32>>()
        );

        fn make_combinations(combinations: &mut Vec<Vec<i32>>, index: usize, n: &usize, k: &usize, overflow: bool) -> () {
            let current_combination: &Vec<i32> = combinations.last().unwrap();

            let is_last_index: bool = index == *k - 1;
            let is_index_value_max: bool = match is_last_index {
                true => current_combination[index] as usize >= *n,
                false => current_combination[index] >= current_combination[index + 1] - 1,
            };

            if is_index_value_max {
                if index > 0 {
                    make_combinations(combinations, index - 1, n, k, true);
                }
                return;
            }

            let mut current_combination: Vec<i32> = current_combination.clone();
            current_combination[index] += 1;
            match overflow {
                true => {
                    for change_index in index + 1..*k {
                        current_combination[change_index] = current_combination[change_index - 1] + 1;
                    }
                    combinations.push(current_combination);
                    make_combinations(combinations, k - 1, n, k, false);
                },
                false => {
                    combinations.push(current_combination);
                    make_combinations(combinations, index, n, k, false);
                },
            };
        }

        make_combinations(&mut combinations, k as usize - 1, &(n as usize), &(k as usize), false);

        return combinations;
    }
}
