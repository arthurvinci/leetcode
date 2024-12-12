struct Solution;
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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let node = root.borrow();
                Self::internal_symmetric(node.left.clone(), node.right.clone())
            }
        }
    }

    fn internal_symmetric(
        left_tree: Option<Rc<RefCell<TreeNode>>>,
        right_tree: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left_tree, right_tree) {
            (Some(left_node), Some(right_node)) => {
                let left = left_node.borrow();
                let right = right_node.borrow();
                left.val == right.val
                    && Self::internal_symmetric(left.left.clone(), right.right.clone())
                    && Self::internal_symmetric(left.right.clone(), right.left.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}