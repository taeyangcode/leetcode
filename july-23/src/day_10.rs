// https://leetcode.com/problems/minimum-depth-of-binary-tree/
// https://leetcode.com/submissions/detail/1008913652/

pub struct Solution;

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
        let root = match root {
            Some(root) => { root.borrow_mut().val = 1; root },
            None => return 0,
        };

        let mut queue: VecDeque<Weak<RefCell<TreeNode>>> = VecDeque::new();

        fn find_minimum_depth(current_node: Weak<RefCell<TreeNode>>, queue: &mut VecDeque<Weak<RefCell<TreeNode>>>) -> i32 {
            let strong_node: Rc<RefCell<TreeNode>> = current_node.upgrade().unwrap();

            let current_value: i32 = strong_node.borrow().val;
            let left_exist: bool = strong_node.borrow().left.is_some();
            let right_exist: bool = strong_node.borrow().right.is_some();

            match (left_exist, right_exist) {
                (false, false) => return strong_node.borrow().val,
                (true, false) => {
                    strong_node.borrow_mut().left.as_mut().unwrap().borrow_mut().val = current_value + 1;
                    queue.push_back(Rc::downgrade(&strong_node.borrow_mut().left.as_mut().unwrap()));
                },
                (false, true) => {
                    strong_node.borrow_mut().right.as_mut().unwrap().borrow_mut().val = current_value + 1;
                    queue.push_back(Rc::downgrade(&strong_node.borrow_mut().right.as_mut().unwrap()));
                },
                (true, true) => {
                    strong_node.borrow_mut().left.as_mut().unwrap().borrow_mut().val = current_value + 1;
                    strong_node.borrow_mut().right.as_mut().unwrap().borrow_mut().val = current_value + 1;

                    queue.push_back(Rc::downgrade(&strong_node.borrow_mut().left.as_mut().unwrap()));
                    queue.push_back(Rc::downgrade(&strong_node.borrow_mut().right.as_mut().unwrap()));
                },
            };

            return find_minimum_depth(queue.pop_front().unwrap(), queue);
        }

        return find_minimum_depth(Rc::downgrade(&root), &mut queue);
    }
}
