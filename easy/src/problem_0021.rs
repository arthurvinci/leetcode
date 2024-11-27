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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(node)) => Some(node),
            (Some(node), None) => Some(node),
            (Some(mut left_node), Some(mut right_node)) => {
                if left_node.val < right_node.val {
                    left_node.next = Self::merge_two_lists(left_node.next, Some(right_node));
                    Some(left_node)
                } else {
                    right_node.next = Self::merge_two_lists(Some(left_node), right_node.next);
                    Some(right_node)
                }
            }
        }
    }
}