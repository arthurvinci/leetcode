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

struct Solution;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(root) => {
                let node = root.borrow();
                match val.cmp(&node.val) {
                    Ordering::Less => Self::search_bst(node.right.clone(), val),
                    Ordering::Equal => {
                        drop(node);
                        Some(root)
                    }
                    Ordering::Greater => Self::search_bst(node.left.clone(), val),
                }
            }
        }
    }
}
