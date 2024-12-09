use std::collections::HashMap;

struct TrieNode {
    index: i32,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            index: -1,
            children: HashMap::new(),
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String, index: i32) {
        let mut current = &mut self.root;

        for char in word.chars() {
            current = current.children.entry(char).or_insert(TrieNode::new());
            current.index = current.index.max(index);
        }
    }

    fn search(&self, word: String) -> i32 {
        let mut current = &self.root;

        for char in word.chars() {
            if let Some(node) = current.children.get(&char) {
                current = node;
            } else {
                return -1;
            }
        }

        current.index
    }
}

struct WordFilter {
    root: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut root = Trie::new();

        for (index, word) in words.iter().enumerate() {
            for i in 0..word.len() {
                root.insert(format!("{}#{}", &word[i..], word), index as i32);
            }
        }

        Self { root }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        self.root.search(format!("{}#{}", suff, pref))
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0745::WordFilter;

    #[test]
    fn test_1() {
        let wf = WordFilter::new(vec!["apple".to_string()]);
        assert_eq!(wf.f("a".to_string(), "e".to_string()), 0);
    }

    #[test]
    fn test_2() {
        let wf = WordFilter::new(vec![
            "apple".to_string(),
            "pineapple".to_string(),
            "aristide".to_string(),
        ]);
        assert_eq!(wf.f("a".to_string(), "e".to_string()), 2);
        assert_eq!(wf.f("ap".to_string(), "r".to_string()), -1);
        assert_eq!(wf.f("a".to_string(), "le".to_string()), 0);
    }
}
