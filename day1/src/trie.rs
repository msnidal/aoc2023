use std::collections::HashMap;

#[derive(Default)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end_of_word: Option<String>,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: None,
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.is_end_of_word = Some(word.to_string());
    }
}
