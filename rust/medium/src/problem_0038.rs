use leetcode_commons::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut even_head = ListNode::new(-1);
        let mut odd_head = ListNode::new(-1);

        let mut even = &mut even_head;
        let mut odd = &mut odd_head;

        let mut count = 0;

        while let Some(mut node) = head {
            count += 1;
            head = std::mem::replace(&mut node.next, None);
            if count % 2 == 1 {
                odd.next = Some(node);
                odd = odd.next.as_mut().unwrap();
            } else {
                even.next = Some(node);
                even = even.next.as_mut().unwrap();
            }
        }
        odd.next = even_head.next;
        odd_head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0038::Solution;
    use leetcode_commons::list_node::ListNode;

    #[test]
    fn test_1() {
        Solution::odd_even_list(Some(Box::new(ListNode::new_with_next(
            1,
            ListNode::new_with_next(
                2,
                ListNode::new_with_next(3, ListNode::new_with_next(4, ListNode::new(5))),
            ),
        ))));
        assert!(false)
    }
}
