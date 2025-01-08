struct Solution;

use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(root) => {
                let mut node = root.borrow_mut();
                match key.cmp(&node.val) {
                    Ordering::Less => {
                        node.left = Self::delete_node(node.left.clone(), key);
                        drop(node);
                        Some(root)
                    }
                    Ordering::Equal => match (node.left.clone(), node.right.clone()) {
                        (None, None) => None,
                        (None, Some(right_node)) => Some(right_node),
                        (Some(left_node), None) => Some(left_node),
                        (Some(left_node), Some(right_node)) => {
                            let new_val = Self::find_min_val(right_node.clone());
                            let mut new_node = TreeNode::new(new_val);
                            new_node.right = Self::delete_node(Some(right_node), new_val);
                            new_node.left = Some(left_node);
                            Some(Rc::new(RefCell::new(new_node)))
                        }
                    },
                    Ordering::Greater => {
                        node.right = Self::delete_node(node.right.clone(), key);
                        drop(node);
                        Some(root)
                    }
                }
            }
        }
    }

    fn find_min_val(root: Rc<RefCell<TreeNode>>) -> i32 {
        let node = root.borrow();
        match node.left.clone() {
            None => node.val,
            Some(left_node) => Self::find_min_val(left_node),
        }
    }
}
