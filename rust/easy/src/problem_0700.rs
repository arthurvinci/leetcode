struct Solution;

use leetcode_commons::tree_node::TreeNode;
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
