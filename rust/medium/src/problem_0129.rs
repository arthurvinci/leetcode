struct Solution;
use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut total = 0;
        Self::sum_numbers_rec(root, 0, &mut total);
        total
    }

    pub fn sum_numbers_rec(root: Option<Rc<RefCell<TreeNode>>>, path: i32, total: &mut i32) {
        if let Some(root) = root {
            let root = root.borrow();
            let new_path = path * 10 + root.val;

            let left_node = root.left.clone();
            let right_node = root.right.clone();
            match (left_node.clone(), right_node.clone()) {
                (None, None) => *total += new_path,
                _ => {
                    Self::sum_numbers_rec(left_node, new_path, total);
                    Self::sum_numbers_rec(right_node, new_path, total);
                }
            }
        }
    }
}
