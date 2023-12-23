/// # anagram module
/// Given a rack, this module will generate a list of anagrams where the
/// score of the estimated score of each anagram is at least 40% of the
/// estimated score of the rack. Supporting functions to return the best
/// anagram & to estimate the score of a given word are also provided.

use crate::agent::rack::Rack;

use super::trieguy::TrieTree;
#[derive(Debug)]
/// ## Anagram
/// A single anagram is made up of the word (String)
/// and the score
pub struct Anagram {
    word: String,
    score: u32,
}

impl Anagram {
    pub fn new(word: String, score: u32) -> Anagram {
        Anagram { word, score }
    }
}

/// ## Anagrams
/// A list of anagrams is made up of a vector of anagrams
pub struct Anagrams {
    anagrams: Vec<Anagram>,
    rack: Rack,
}

impl Iterator for Anagrams {
    type Item = Anagram;

    fn next(&mut self) -> Option<Self::Item> {
        self.anagrams.pop() // Remove and return the last anagram
    }
}

/// # Anagrams class instantiates with a list of empty Anagrams
/// the generate() function fills that list and get_best_anagram
/// returns the the Anagram with the highest score.
/// ## Usage
/// Anagrams anagrams = new Anagrams(rack)
/// let winner = anagrams.get_best_anagram()
/// println!("Winner: {}", winner.word)
impl Anagrams {
    pub fn new(rack: Rack) -> Self {
        Anagrams {
            anagrams: Vec::new(),
            rack: rack.clone(),
        }
    }
    /// ## get_best_anagram()
    /// Returns the Anagram with the highest score
    /// removing it from the object; freeing up the
    /// next best anagram to be returned
    pub fn get_best_anagram(&self) -> &Anagram {
        self.anagrams.get(0).unwrap().clone()
    }
    /// ## estimate_score()
    /// Given a string, this will estimate the highest score
    /// possible for that word.
    fn estimate_score(&self, word: String) -> u32 {
        let mut score: u32 = 0;
        let mut local_max: u32 = 0;
        let weighted_groups = &self.rack.weighted_groups;
        for letter in word.chars() {
            for group in weighted_groups {
                if group.letters.contains(&letter) {
                    if local_max < group.weight {
                        local_max = group.weight;
                    }
                    score += group.weight;
                }
            }
        }
        score += local_max;
        score
    }
    /// ## Generate
    /// This is where the magic happens. We recursively call backtrack() until a word
    /// is found. If we've got a word and its a high scoring word, add it to the list.
    /// Otherwise, we keep walking down the TrieTree until a word is found. We handle
    /// wildcards by ordering the alphabet by frequency of letters, and handling them
    /// one at a time.
    pub fn generate(&mut self, path: String, rack: &mut Rack, trie_guy: &mut TrieTree) {
        let word = path.clone();
        let max_score: u32 = (rack.max_score() as f32 * 0.40).floor() as u32;

        if trie_guy.search(&word) {
            let score = self.estimate_score(word.clone());
            if score >= max_score {
                self.anagrams.push(Anagram::new(word, score));
            }
        }

        for (i, char) in rack.letters.iter().enumerate() {
            // If we have a wildcard
            if *char == '_' {
                for common_char in "AEIOULNRDBCGMFHPSVWYJQKZ".chars() {
                    // Create a new path for each recursive call
                    let mut new_path = path.clone();
                    new_path.push(common_char);
                    // Create a new rack for each recursive call
                    let mut new_rack = rack.clone();
                    new_rack.letters.remove(i);
                    // Start over less one letter
                    self.generate(new_path, &mut new_rack, trie_guy);
                }
            } else {
                // Create a new path for each recursive call
                let mut new_path = path.clone();
                new_path.push(*char);
                // Create a new rack for each recursive call
                let mut new_rack = rack.clone();
                new_rack.letters.remove(i);
                // Start over less one letter
                self.generate(new_path, &mut new_rack, trie_guy);
            }
        }
    }
}
