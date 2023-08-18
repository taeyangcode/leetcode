// https://leetcode.com/problems/unique-binary-search-trees-ii/

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut cache: HashMap<(i32, i32), Option<Vec<Node>>> = HashMap::new();
        let values: Vec<i32> = (1..=n).collect::<Vec<i32>>();


        fn generate(cache: &mut HashMap<(i32, i32), Option<Vec<Node>>>, values: &Vec<i32>, pivot: usize, left_index: usize, right_index: usize) {
            let tree_node = |value: i32| Rc::new(RefCell::new(TreeNode::new(value)));

            let left_tree: Option<Vec<Node>> = match cache.get(&(left_index as i32, pivot as i32)) {
                Option::Some(left_tree) => left_tree.clone(),
                Option::None => {
                    let left_values: &[i32] = &values[left_index..pivot as usize];

                    match left_values.len() {
                        0 => { cache.insert((left_index as i32, pivot as i32), Option::None); },
                        1 => { cache.insert((left_index as i32, pivot as i32), Option::Some(vec![tree_node(left_values[0])])); },
                        _ => {
                            for left_value in left_values {

                            }
                        },
                    };
                    cache.get(&(left_index as i32, pivot as i32)).unwrap().clone()
                },
            };

            let right_values: &[i32] = &values[pivot as usize..right_index];
        }

        todo!();
    }
}
