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
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 0;
        Self::internal_diameter(root, &mut diameter);
        diameter
    }

    fn internal_diameter(root: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> u32 {
        match root {
            None => 0,
            Some(root) => {
                let node = root.borrow();
                let right_height = Self::internal_diameter(node.right.clone(), diameter);
                let left_height = Self::internal_diameter(node.left.clone(), diameter);
                let new_diameter = (left_height + right_height) as i32;
                if new_diameter > *diameter {
                    *diameter = new_diameter
                }

                right_height.max(left_height) + 1
            }
        }
    }
}