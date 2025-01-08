struct Solution;

use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_rec(
            &preorder,
            0,
            preorder.len() - 1,
            &inorder,
            0,
            inorder.len() - 1,
        )
    }

    fn build_tree_rec(
        preorder: &[i32],
        left_preorder: usize,
        right_preorder: usize,
        inorder: &[i32],
        left_inorder: usize,
        right_inorder: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut new_node = TreeNode::new(preorder[left_preorder]);

        let mut new_inorder_index = left_inorder;
        for index in left_inorder..=right_inorder {
            if inorder[index] == new_node.val {
                new_inorder_index = index;
                break;
            }
        }

        let left_sub_nodes = new_inorder_index - left_inorder;
        if new_inorder_index > left_inorder {
            new_node.left = Self::build_tree_rec(
                preorder,
                left_preorder + 1,
                left_preorder + left_sub_nodes,
                inorder,
                left_inorder,
                new_inorder_index - 1,
            );
        }

        if new_inorder_index < right_inorder {
            new_node.right = Self::build_tree_rec(
                preorder,
                left_preorder + left_sub_nodes + 1,
                right_preorder,
                inorder,
                new_inorder_index + 1,
                right_inorder,
            )
        }

        Some(Rc::new(RefCell::new(new_node)))
    }
}
