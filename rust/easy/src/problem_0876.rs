struct Solution;
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
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.clone();
        let mut fast = head;

        while fast.is_some() && fast.clone().and_then(|node| node.next).is_some() {
            fast = fast.and_then(|node| node.next).and_then(|node| node.next);
            slow = slow.and_then(|node| node.next)
        }

        slow
    }
}
