use std::collections::HashMap;

struct Trie {
    root: TrieNode,
}

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_word: false,
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut trie_node = &mut self.root;
        for char in word.chars() {
            if trie_node.children.get(&char).is_none() {
                trie_node.children.insert(char, TrieNode::new());
            }

            trie_node = trie_node.children.get_mut(&char).unwrap();
        }
        trie_node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        match self.get_node(word) {
            None => false,
            Some(node) => node.is_word,
        }
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.get_node(prefix).is_some()
    }

    fn get_node(&self, word: String) -> Option<&TrieNode> {
        let mut trie_node = &self.root;
        for char in word.chars() {
            match trie_node.children.get(&char) {
                None => return None,
                Some(trie) => trie_node = trie,
            }
        }
        Some(trie_node)
    }
}
