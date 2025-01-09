use leetcode_commons::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut next_head = head.as_mut();
        for _ in 0..k - 1 {
            if let Some(node) = next_head {
                next_head = node.next.as_mut();
            } else {
                return head;
            }
        }
        if let Some(node) = next_head {
            let next = node.next.take();
            let reversed_next = Self::reverse_k_group(next, k);
            Self::do_reverse(head, reversed_next, None)
        } else {
            head
        }
    }

    pub fn do_reverse(
        mut head: Option<Box<ListNode>>,
        point_to: Option<Box<ListNode>>,
        acc: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => acc,
            Some(mut node) => {
                let next = node.next.take();
                if acc.is_none() {
                    node.next = point_to;
                    Self::do_reverse(next, None, Some(node))
                } else {
                    node.next = acc;
                    Self::do_reverse(next, point_to, Some(node))
                }
            }
        }
    }
}
