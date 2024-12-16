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

    fn new_with_next(val: i32, next: ListNode) -> Self {
        Self {
            val,
            next: Some(Box::new(next)),
        }
    }
}
impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut pre = &mut dummy;
        for _ in 1..left {
            pre = &mut pre.as_mut().unwrap().next;
        }
        let mut cur = pre.as_mut().unwrap().next.take();
        for _ in 0..right - left + 1 {
            let next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = pre.as_mut().unwrap().next.take();
            pre.as_mut().unwrap().next = cur.take();
            cur = next;
        }
        for _ in 0..right - left + 1 {
            pre = &mut pre.as_mut().unwrap().next;
        }
        pre.as_mut().unwrap().next = cur;
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0092::{ListNode, Solution};

    #[test]
    fn test_1() {
        let head = ListNode::new_with_next(
            1,
            ListNode::new_with_next(
                2,
                ListNode::new_with_next(3, ListNode::new_with_next(4, ListNode::new(5))),
            ),
        );

        Solution::reverse_between(Some(Box::new(head)), 2, 4);
    }
}
