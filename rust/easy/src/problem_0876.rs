use leetcode_commons::list_node::ListNode;

struct Solution;
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
