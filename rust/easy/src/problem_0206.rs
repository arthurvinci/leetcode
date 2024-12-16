#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_list_inner(head, None)
    }

    fn reverse_list_inner(
        head: Option<Box<ListNode>>,
        new_head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => new_head,
            Some(mut node) => {
                let new_tree = node.next;
                node.next = new_head;
                Self::reverse_list_inner(new_tree, Some(node))
            }
        }
    }
}
