use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (Some(l_n), Some(r_n)) => {
                let l_node = l_n.borrow();
                let r_node = r_n.borrow();

                l_node.val == r_node.val
                    && Self::is_same_tree(l_node.left.clone(), r_node.left.clone())
                    && Self::is_same_tree(l_node.right.clone(), r_node.right.clone())
            }
        }
    }
}
