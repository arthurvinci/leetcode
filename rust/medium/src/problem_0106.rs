struct Solution;
use leetcode_commons::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_rec(
            &inorder,
            0,
            inorder.len() - 1,
            &postorder,
            0,
            postorder.len() - 1,
        )
    }

    fn build_tree_rec(
        inorder: &[i32],
        left_inorder: usize,
        right_inorder: usize,
        postorder: &[i32],
        left_postorder: usize,
        right_postorder: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut new_node = TreeNode::new(postorder[right_postorder]);

        let mut new_node_inorder_index = left_inorder;
        for i in left_inorder..=right_inorder {
            if inorder[i] == new_node.val {
                new_node_inorder_index = i;
                break;
            }
        }
        let right_sub_nodes = right_inorder - new_node_inorder_index;
        if new_node_inorder_index > left_inorder {
            new_node.left = Self::build_tree_rec(
                inorder,
                left_inorder,
                new_node_inorder_index - 1,
                postorder,
                left_postorder,
                right_postorder - right_sub_nodes - 1,
            );
        }

        let left_sub_nodes = new_node_inorder_index - left_inorder;
        if new_node_inorder_index < right_inorder {
            new_node.right = Self::build_tree_rec(
                inorder,
                new_node_inorder_index + 1,
                right_inorder,
                postorder,
                left_postorder + left_sub_nodes,
                right_postorder - 1,
            )
        }

        Some(Rc::new(RefCell::new(new_node)))
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0106::Solution;

    #[test]
    fn test_1() {
        let ok = Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);
        println!("{:?}", ok);
        panic!()
    }
}
