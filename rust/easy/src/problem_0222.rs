struct Solution;
use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let node = root.borrow();
                1 + Self::count_nodes(node.left.clone()) + Self::count_nodes(node.right.clone())
            }
        }
    }
}
