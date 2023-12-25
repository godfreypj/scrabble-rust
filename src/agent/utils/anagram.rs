/// # anagram module
/// Given a rack, this module will generate a list of anagrams where the
/// score of the estimated score of each anagram is at least 40% of the
/// estimated score of the rack. Supporting functions to return the best
/// anagram & to estimate the score of a given word are also provided.
use std::{cmp::Ordering, collections::HashMap};
use crate::agent::rack::Rack;
use super::trieguy::TrieTree;
#[derive(Debug)]

// ## Move
// A simple struct that holds the result of
// the find_best_move() function
pub struct Move {
    pub word: String,
    pub starting_element: u32,
}

impl Move {
    pub fn new(word: String, starting_element: u32) -> Self {
        Move {
            word,
            starting_element,
        }
    }
}
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
    pub fn get_best_anagram(&mut self) -> Option<&Anagram> {
        self.anagrams.sort_by(|a, b| {
            let score_cmp = b.score.cmp(&a.score); // Descending order for scores
            if score_cmp != Ordering::Equal {
                score_cmp
            } else {
                b.word.len().cmp(&a.word.len()) // Descending order for word length
            }
        });
        return self.anagrams.first() // Return the first (best) anagram
    }
    
    /// ## estimate_score()
    /// Given a string, this will estimate the highest score
    /// possible for that word.
    fn estimate_score(&self, word: String) -> u32 {
        let mut score: u32 = 0;
        let mut local_max: u32 = 0;
        for letter in word.chars() {
            for group in &self.rack.weighted_groups.weighted_groups {
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
    /// This is where the magic happens. We recursively call generate() until a word
    /// of at least 4 characters is found. If we've got a word and its a high scoring word, 
    /// add it to the list.
    /// Otherwise, we keep walking down the TrieTree until a word is found. We handle
    /// wildcards by ordering the alphabet by frequency of letters, and handling them
    /// one at a time.
    pub fn generate(&mut self, path: String, rack: &mut Rack, trie_guy: &mut TrieTree) {
        let word = path.clone();
        let max_score: u32 = (rack.max_score() as f32 * 0.40).floor() as u32;

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

    /// ## find_best_move()
    /// For every letter more than 5 characters, the letters that can land on
    /// the double letter score increases. Meaning if its 5 characters, the first
    /// or last letter must start on the double letter score in order for the word
    /// to cover the center square. As the word gets longer, we can try more and more
    /// letters on the double letter score.
    pub fn find_best_move(mut self) -> Move {
        // If the word is less than 5 characters it must start on center square
        let anagram = self.get_best_anagram().unwrap();
        let word = anagram.word.clone();
        let mut index_vs_weight_hashmap: HashMap<usize, u32> = HashMap::new();
        for letter in word.chars() {
            let letter_weight = self.rack.weighted_groups.get_weight(letter);
            let index = word.chars().position(|c| c == letter).unwrap();
            index_vs_weight_hashmap.insert(index, letter_weight);
        }
        let highest_weighted_index = index_vs_weight_hashmap.iter().max_by_key(|x| x.1).unwrap().0;
        if word.len() < 5 {
            return Move::new(word, 6);
        }
        // If its 5 characters
        // Check the first and last letter, whichever is higher determines the placement
        // If its the first letter, place starting on element 3; otherwise start on center square
        else if word.len() == 5 {
            if *highest_weighted_index == 0 {
                return Move::new(word, 3);
            } else {
                return Move::new(word, 6);
            }
        }
        // If its 6 characters
        // Check first and second letter, then the last and 2nd to last
        // Find the highest scoring letter. If its the first or second
        // place the worst so the highest scoring letter is on element 3; 
        // otherwise place the word so the highest scoring letter is on element 10
        else if word.len() == 6 {
            if *highest_weighted_index == 0 {
                return Move::new(word, 3);
            } else if *highest_weighted_index == 1 {
                return Move::new(word, 2);
            } else if *highest_weighted_index == 5 {
                return Move::new(word, 5);
            } else {
                return Move::new(word, 4);
            }
        }
        // Otherwise, its 7 letters
        // Repeat the process for the first 3 and last 3 letters
        else {
            if *highest_weighted_index == 0 {
                return Move::new(word, 3);
            } else if *highest_weighted_index == 1 {
                return Move::new(word, 2);
            } else if *highest_weighted_index == 2 {
                return Move::new(word, 1);
            } else if *highest_weighted_index == 5 {
                return Move::new(word, 5);
            } else if *highest_weighted_index == 4 {
                return Move::new(word, 4);
            } else {
                return Move::new(word, 3);
            }
        }
    }
}
