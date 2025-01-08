use leetcode_commons::list_node::ListNode;

struct Solution;
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            None
        } else if lists.len() == 1 {
            lists[0].clone()
        } else {
            let right_lists = lists.split_off(lists.len() / 2);
            Self::merge_two_lists(Self::merge_k_lists(lists), Self::merge_k_lists(right_lists))
        }
    }

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
