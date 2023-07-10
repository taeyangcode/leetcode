// https://leetcode.com/problems/minimum-depth-of-binary-tree/

pub struct Solution;

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
use std::collections::VecDeque;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue: VecDeque<Weak<RefCell<TreeNode>>> = VecDeque::new();

        fn find_min_depth(node: Rc<RefCell<TreeNode>>, queue: &mut VecDeque<Weak<RefCell<TreeNode>>>) -> i32 {
            let borrow_node = node.borrow();
            let left_node = borrow_node.left.as_ref().map(Rc::downgrade);
            let right_node = borrow_node.right.as_ref().map(Rc::downgrade);

            match (left_node, right_node) {
                (Some(left_node), Some(right_node)) => {
                    queue.push_back(left_node);
                    queue.push_back(right_node);
                },
                (Some(left_node), None) => queue.push_back(left_node),
                (None, Some(right_node)) => queue.push_back(right_node),
                (None, None) => return 1,
            };

            1 + find_min_depth(queue.pop_front().unwrap().upgrade().unwrap(), queue)
        }

        match root {
            Some(node) => find_min_depth(node, &mut queue),
            None => 0,
        }
    }
}
