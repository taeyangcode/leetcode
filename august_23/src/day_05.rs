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

use std::rc::{Rc, Weak};
use std::cell::RefCell;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        type ResultType = Vec<Option<Rc<RefCell<TreeNode>>>>;
        type Node = RefCell<TreeNode>;

        let mut result: ResultType = vec![];

        fn generate_tree(trees: &mut ResultType, head_node: Weak<Node>, current_node: Weak<Node>, count: i32) {
        }

        return result;
    }
}
