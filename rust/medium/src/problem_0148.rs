use leetcode_commons::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::merge_sort(head)
    }

    fn merge_sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            head
        } else {
            let (first_half, second_half) = Self::split(head);
            Self::merge(Self::merge_sort(first_half), Self::merge_sort(second_half))
        }
    }

    fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return (head, None);
        }

        let mut fast = head.clone().and_then(|node| node.next);
        let mut slow = head.as_mut();

        while fast.is_some() && fast.as_ref().and_then(|node| node.next.as_ref()).is_some() {
            fast = fast.and_then(|node| node.next.and_then(|node| node.next));
            slow = slow.and_then(|node| node.next.as_mut());
        }

        let second_half = slow.and_then(|node| node.next.take());

        (head, second_half)
    }

    fn merge(
        mut head1: Option<Box<ListNode>>,
        mut head2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut dummy;

        while head1.is_some() || head2.is_some() {
            if head1.is_some()
                && (head2.is_none() || head1.as_ref().unwrap().val <= head2.as_ref().unwrap().val)
            {
                let next = head1.as_mut().unwrap().next.take();
                tail.next = head1;
                tail = tail.next.as_mut().unwrap();
                head1 = next;
            } else if head2.is_some() {
                let next = head2.as_mut().unwrap().next.take();
                tail.next = head2;
                tail = tail.next.as_mut().unwrap();
                head2 = next;
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0148::{ListNode, Solution};

    #[test]
    fn test_1() {
        let ret = Solution::sort_list(Some(Box::new(ListNode::new_with_next(
            4,
            ListNode::new_with_next(2, ListNode::new_with_next(1, ListNode::new(3))),
        ))));
        println!("{:?}", ret);
    }
}
