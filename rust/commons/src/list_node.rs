#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn new_with_next(val: i32, next: ListNode) -> Self {
        Self {
            val,
            next: Some(Box::new(next)),
        }
    }
}
