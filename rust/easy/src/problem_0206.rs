use leetcode_commons::list_node::ListNode;

struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_list_inner(head, None)
    }

    fn reverse_list_inner(
        head: Option<Box<ListNode>>,
        new_head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => new_head,
            Some(mut node) => {
                let new_tree = node.next;
                node.next = new_head;
                Self::reverse_list_inner(new_tree, Some(node))
            }
        }
    }
}
