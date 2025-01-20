struct Solution;

use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        Self::count_good_nodes(root, &mut count, i32::MIN);
        count
    }

    fn count_good_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        count: &mut i32,
        biggest_in_path: i32,
    ) {
        if let Some(root) = root {
            let root = root.borrow();
            if root.val >= biggest_in_path {
                *count += 1;
            }

            Self::count_good_nodes(root.left.clone(), count, biggest_in_path.max(root.val));
            Self::count_good_nodes(root.right.clone(), count, biggest_in_path.max(root.val));
        }
    }
}
