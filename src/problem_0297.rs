#[derive(Debug, PartialEq, Eq, Clone)]
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
use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => "X".to_string(),
            Some(internal_root) => {
                let node = internal_root.borrow();
                format!(
                    "{};{};{}",
                    node.val,
                    self.serialize(node.left.clone()),
                    self.serialize(node.right.clone())
                )
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let array: Vec<&str> = data.split(";").collect();
        let mut index = 0;
        self.internal_deserialize(&array, &mut index)
    }

    fn internal_deserialize(
        &self,
        nodes: &[&str],
        index: &mut usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match nodes[*index] {
            "X" => {
                *index += 1;
                None
            }
            value => {
                *index += 1;
                let mut node = TreeNode::new(value.parse::<i32>().unwrap());
                node.left = self.internal_deserialize(nodes, index);
                node.right = self.internal_deserialize(nodes, index);
                Some(Rc::new(RefCell::new(node)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0297::{Codec, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_1() {
        let nodes_value = vec![1, 2, 3, 4];
        let mut nodes: Vec<TreeNode> = nodes_value
            .iter()
            .map(|value| TreeNode::new(value.clone()))
            .collect();

        nodes[0].right = Some(Rc::new(RefCell::new(nodes[2].clone())));

        let mut left_tree = nodes[1].clone();
        left_tree.left = Some(Rc::new(RefCell::new(nodes[3].clone())));

        nodes[0].left = Some(Rc::new(RefCell::new(left_tree)));

        let tree = Some(Rc::new(RefCell::new(nodes[0].clone())));

        let serializer = Codec::new();
        let serialized = serializer.serialize(tree.clone());
        let deserialized = serializer.deserialize(serialized);

        assert_eq!(tree, deserialized);
    }
}
