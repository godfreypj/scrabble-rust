use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// # TrieNode
/// Each node of the TrieGuy contains a child, or it
/// contains the end of the word boolean set to true.
/// The children are stored in a HashMap.
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

/// # TrieTree
/// A TrieTree is a data structure that loads up the dictionary
/// of scrabble words. It's a convenient data struct for generating
/// anagrams because it allows for quick lookups.
/// Functions:
/// - `load_dictionoary()`, `insert()`, `search()`
pub(crate) struct TrieTree {
    pub root: Box<TrieNode>,
}

impl TrieTree {
    pub fn new() -> Self {
        let root = TrieTree::load_dictionary("src/data/dictionary.txt");
        TrieTree {
            root: Box::new(root),
        }
    }

    /// ### load_dictionary()
    /// Given a text file this function will open and
    /// create a new reader object. It will then iterate
    /// over each line and convert it to a String. It will
    /// then pass the word into the `insert()` function.
    fn load_dictionary(filename: &str) -> TrieNode {
        let file = File::open(filename).expect("Failed to open dictionary file");
        let reader = BufReader::new(file);
        let mut root = TrieNode::new();
        for line in reader.lines() {
            let word = line.unwrap().trim().to_string();
            TrieTree::insert(&mut root, &word);
        }
        root
    }

    /// ### insert()
    /// This function iterates over a given letter, for each
    /// letter, it checks if the letter is in the tree. If
    /// it is not, it adds the letter to the tree. The last node
    /// is marked as the end of the word.
    pub fn insert(root: &mut TrieNode, word: &str) {
        let mut node = root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(Box::new(TrieNode::new()));
        }
        node.is_end_of_word = true;
    }

    /// ### search()
    /// Given a `word`, iterate over each character
    /// working your way down the tree. If the character
    /// is not in the tree, return `false`. Otherwise
    /// return `true`.
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
