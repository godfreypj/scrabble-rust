use crate::{
    agent::rack::Rack,
    utils::{trieguy::TrieTree, weighted_group::WeightedGroups},
};

/// # Anagram
/// A single anagram is made up of the word (String) and the score.
#[derive(Clone)]
pub struct Anagram {
    pub word: String,
    pub score: u32,
}

impl Anagram {
    pub fn new(word: String, score: u32) -> Anagram {
        Anagram { word, score }
    }
}

/// # Anagrams
/// Given a rack, this module will generate a list of anagrams and its
/// estimated score. Each anagram is at least 40% of the
/// max possible score of the rack.
/// Functions:
/// - `get_best_anagram()`, `estimate_score()`, `generate()`
pub struct Anagrams {
    anagrams: Vec<Anagram>,
}

impl Iterator for Anagrams {
    type Item = Anagram;
    fn next(&mut self) -> Option<Self::Item> {
        self.anagrams.pop()
    }
}

impl Anagrams {
    pub fn new() -> Self {
        Anagrams {
            anagrams: Vec::new(),
        }
    }
    /// ### get_best_anagram()
    /// Returns the `Anagram` with the highest score
    /// removing it from the object; freeing up the
    /// next best anagram to be returned
    // pub fn get_best_anagram(&mut self) -> Option<&Anagram> {
    //     self.anagrams.sort_by(|a, b| {
    //         let score_cmp = b.score.cmp(&a.score); // Descending order for scores
    //         if score_cmp != Ordering::Equal {
    //             score_cmp
    //         } else {
    //             b.word.len().cmp(&a.word.len()) // Descending order for word length
    //         }
    //     });
    //     return self.anagrams.first(); // Return the first (best) anagram
    // }

    /// ### estimate_score()
    /// Given a string, this will estimate the highest score
    /// possible for that word.
    fn estimate_score(&self, word: String) -> u32 {
        let mut score: u32 = 0;
        let mut local_max: u32 = 0;
        let wg: WeightedGroups = WeightedGroups::new();
        let score_groups = wg.score_groups.clone();
        // If the word is less than
        for letter in word.chars() {
            for group in score_groups.clone() {
                if group.letters.contains(&letter) {
                    if local_max < group.weight {
                        local_max = group.weight;
                    }
                    score += group.weight;
                }
            }
        }
        // If its possible to hit the double letter square, double it
        if word.len() > 4 {
            score += local_max;
        }
        score
    }

    /// ### generate()
    /// This is where the magic happens. We recursively call `generate()` until a word
    /// of at least 4 characters is found. If we've got a word and its a high scoring word,
    /// add it to the list.
    /// Otherwise, we keep walking down the tree until a word is found. We handle
    /// wildcards by ordering the alphabet by frequency of letters.
    pub fn generate(&mut self, path: String, rack: &mut Rack, trie_guy: &mut TrieTree) {
        let word = path.clone();
        let rack_word: String = rack.letters.iter().collect();
        let max_score: u32 = (self.estimate_score(rack_word) as f32 * 0.40).floor() as u32;

        if trie_guy.search(&word) && word.len() >= 4 {
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
