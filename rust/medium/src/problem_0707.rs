struct MyLinkedList {
    head: Option<Box<MyLinkedListNode>>,
    len: i32,
}

#[derive(Debug, Clone)]
struct MyLinkedListNode {
    val: i32,
    next: Option<Box<MyLinkedListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        Self { head: None, len: 0 }
    }

    fn get(&mut self, index: i32) -> i32 {
        match self.get_node_at_index(index) {
            None => -1,
            Some(node) => node.val,
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let new_node = MyLinkedListNode {
            val,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
        self.len += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.len, val);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.add_at_head(val)
        } else {
            let opt_node = self.get_node_at_index(index - 1);

            if let Some(ref mut node) = opt_node {
                let new_node = MyLinkedListNode {
                    val,
                    next: node.next.take(),
                };

                node.next = Some(Box::new(new_node));
                self.len += 1;
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index == 0 {
            match self.head.take() {
                None => {}
                Some(node) => {
                    self.head = node.next;
                    self.len -= 1
                }
            }
        } else {
            let opt_node = self.get_node_at_index(index - 1);
            if let Some(node) = opt_node {
                let to_delete = node.next.take();

                if let Some(to_delete) = to_delete {
                    node.next = to_delete.next;
                    self.len -= 1;
                }
            }
        }
    }

    fn get_node_at_index(&mut self, index: i32) -> &mut Option<Box<MyLinkedListNode>> {
        let mut current = &mut self.head;
        for _ in 0..index {
            if current.is_none() {
                return current;
            }

            current = &mut current.as_mut().unwrap().next;
        }

        current
    }

    fn to_string(&self) {
        let mut stringified = String::new();
        let mut head = &self.head;
        while head.is_some() {
            let head_val = head.as_ref().unwrap();
            stringified = format!("{} -> {}", stringified, head_val.val);
            head = &head_val.next;
        }
        println!("{}", stringified)
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0707::MyLinkedList;

    #[test]
    fn test_1() {
        let mut linked_list = MyLinkedList::new();
        linked_list.add_at_head(1);
        linked_list.to_string();
        linked_list.add_at_tail(3);
        linked_list.to_string();
        linked_list.add_at_index(1, 2);
        linked_list.to_string();
        assert_eq!(linked_list.get(1), 2);
        linked_list.delete_at_index(0);
        linked_list.to_string();
        assert_eq!(linked_list.get(1), 3);
    }
}
