// https://leetcode.com/problems/permutations/
// https://leetcode.com/submissions/detail/1012486394/

struct Solution;

use std::collections::HashSet;

impl Solution {
    fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cache: HashSet<Vec<i32>> = HashSet::new();

        fn find_permutations(cache: &mut HashSet<Vec<i32>>, length: &usize, permutation: &Vec<i32>, index: usize) -> () {
            if index >= *length - 1 {
                cache.insert(permutation.clone());
                return;
            }
            if cache.get(permutation).is_some() {
                return;
            }

            for array_index in index..*length {
                let mut new_permutation: Vec<i32> = permutation.clone();
                let temporary: i32 = new_permutation[index];
                new_permutation[index] = new_permutation[array_index];
                new_permutation[array_index] = temporary;

                find_permutations(cache, length, &new_permutation, index + 1);
            }
        }

        find_permutations(&mut cache, &nums.len(), &nums, 0);

        return cache.into_iter().collect::<Vec<Vec<i32>>>();
    }
}
