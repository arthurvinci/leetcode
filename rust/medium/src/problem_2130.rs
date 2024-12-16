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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let (mut start, mut middle) = Self::split_in_half(head);
        middle = Self::reverse(middle, None);

        let mut max_twins = i32::MIN;

        loop {
            match (start, middle) {
                (None, None) => break,
                (None, Some(node)) | (Some(node), None) => {
                    max_twins = max_twins.max(node.val);
                    break;
                }
                (Some(node_1), Some(node_2)) => {
                    max_twins = max_twins.max(node_2.val + node_1.val);
                    start = node_1.next;
                    middle = node_2.next;
                }
            }
        }

        max_twins
    }

    pub fn reverse(
        head: Option<Box<ListNode>>,
        new_head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => new_head,
            Some(mut node) => {
                let sub_tree = node.next.take();
                node.next = new_head;
                Self::reverse(sub_tree, Some(node))
            }
        }
    }

    pub fn split_in_half(
        head: Option<Box<ListNode>>,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut fast = head.clone();
        let mut slow = head.clone();

        while fast.is_some() && fast.clone().and_then(|node| node.next).is_some() {
            fast = fast.and_then(|node| node.next).and_then(|node| node.next);
            slow = slow.and_then(|node| node.next)
        }

        (head, slow)
    }
}
