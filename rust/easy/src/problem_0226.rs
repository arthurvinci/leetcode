use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let mut b_node = node.borrow_mut();
                let new_left = Self::invert_tree(b_node.right.clone());
                let new_right = Self::invert_tree(b_node.left.clone());
                b_node.left = new_left;
                b_node.right = new_right;
                drop(b_node);

                Some(node)
            }
        }
    }
}
