use leetcode_commons::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &(head.clone());
        let mut slow = &mut head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &mut (slow.as_mut()?.next);
            fast = &(fast.as_ref()?.next.as_ref()?.next);
        }

        *slow = (*slow).as_mut()?.next.take();

        head
    }
}
