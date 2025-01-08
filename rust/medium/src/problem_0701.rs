use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(root) => {
                let mut node = root.borrow_mut();
                if val > node.val {
                    node.right = Self::insert_into_bst(node.right.clone(), val);
                } else {
                    node.left = Self::insert_into_bst(node.left.clone(), val);
                }
                drop(node);
                Some(root)
            }
        }
    }
}
