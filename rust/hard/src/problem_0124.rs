struct Solution;

use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (max_branch, max_path) = Self::max_path_sum_intern(root.unwrap());
        max_path.max(max_branch)
    }

    fn max_path_sum_intern(root: Rc<RefCell<TreeNode>>) -> (i32, i32) {
        let root = root.borrow();
        let (left_branch, left_max_path) = root
            .left
            .clone()
            .map(|node| Self::max_path_sum_intern(node))
            .unwrap_or((i32::MIN, i32::MIN));
        let (right_branch, right_max_path) = root
            .right
            .clone()
            .map(|node| Self::max_path_sum_intern(node))
            .unwrap_or((i32::MIN, i32::MIN));

        let max_branch = left_branch.max(right_branch).saturating_add(root.val);

        let local_paths = left_max_path.max(right_max_path);

        let global_path = right_branch.max(left_branch).max(
            right_branch
                .saturating_add(left_branch)
                .saturating_add(root.val),
        );

        (
            max_branch.max(root.val),
            local_paths.max(global_path).max(root.val),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0124::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_1() {
        let root = TreeNode::new(-3);
        assert_eq!(
            Solution::max_path_sum(Some(Rc::new(RefCell::new(root)))),
            -3
        )
    }
}
