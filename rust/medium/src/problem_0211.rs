use std::collections::HashMap;
use std::str::Chars;

struct WordDictionary {
    root: PrefixNode,
}

struct PrefixNode {
    children: HashMap<char, PrefixNode>,
    is_word: bool,
}

impl PrefixNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self {
            root: PrefixNode::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut prefix_node = &mut self.root;
        for char in word.chars() {
            if prefix_node.children.get(&char).is_none() {
                prefix_node.children.insert(char, PrefixNode::new());
            }

            prefix_node = prefix_node.children.get_mut(&char).unwrap();
        }
        prefix_node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        self.search_start_at(word.chars(), &self.root)
    }
    fn search_start_at(&self, mut chars: Chars, prefix_node: &PrefixNode) -> bool {
        match chars.next() {
            None => prefix_node.is_word,
            Some(ch) => {
                if ch == '.' {
                    for prefix_node in prefix_node.children.values() {
                        if self.search_start_at(chars.clone(), prefix_node) {
                            return true;
                        }
                    }

                    false
                } else {
                    match prefix_node.children.get(&ch) {
                        None => false,
                        Some(prefix) => self.search_start_at(chars, prefix),
                    }
                }
            }
        }
    }
}
