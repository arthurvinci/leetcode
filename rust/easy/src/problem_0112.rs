struct Solution;

use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(root) => {
                let node = root.borrow();
                let new_val = target_sum - node.val;

                match (node.left.clone(), node.right.clone()) {
                    (None, None) => new_val == 0,
                    (left_node, right_node) => {
                        Self::has_path_sum(left_node, new_val)
                            || Self::has_path_sum(right_node, new_val)
                    }
                }
            }
        }
    }
}
