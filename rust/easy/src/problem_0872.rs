struct Solution;

use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::get_leaf_sequence(root1) == Self::get_leaf_sequence(root2)
    }

    pub fn get_leaf_sequence(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let root = root.borrow();

                let mut left_vec = Self::get_leaf_sequence(root.left.clone());
                let mut right_vec = Self::get_leaf_sequence(root.right.clone());

                if left_vec.is_empty() && right_vec.is_empty() {
                    left_vec.push(root.val);
                } else {
                    left_vec.append(&mut right_vec);
                }

                left_vec
            }
        }
    }
}
