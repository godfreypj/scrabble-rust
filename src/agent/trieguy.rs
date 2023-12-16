// agent/trieguy.rs
//
// Module: Trie Guy
// this module will read in the dictionary text provided and create a Trie Tree
// of all the words for the agent to use when generating anagrams.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct TrieNode {
    pub children: HashMap<char, Box<TrieNode>>,
    pub is_end_of_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

pub(crate) struct TrieTree {
    pub root: Box<TrieNode>,
}

impl TrieTree {
    pub fn new() -> Self {
        let root = TrieTree::load_dictionary("src/agent/dictionary.txt");
        TrieTree {
            root: Box::new(root),
        }
    }

    fn load_dictionary(filename: &str) -> TrieNode {
        let file = File::open(filename).expect("Failed to open dictionary file");
        let reader = BufReader::new(file);
        let mut root = TrieNode::new();
        for line in reader.lines() {
            let word = line.unwrap().trim().to_string(); // Convert each line to a String
            TrieTree::insert(&mut root, &word);
        }
        root
    }

    pub fn insert(root: &mut TrieNode, word: &str) {
        let mut node = root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(Box::new(TrieNode::new()));
        }
        node.is_end_of_word = true;
    }
    

    pub fn search(&mut self, word: &str) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            if !node.children.contains_key(&ch) {
                return false;
            }
            node = &*node.children.get(&ch).unwrap();
        }
        node.is_end_of_word
    }
}