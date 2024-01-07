use std::collections::HashMap;

use crate::agent::rack::Rack;
use crate::agent::scrabble_move::ScrabbleMove;
use crate::anagrams::anagrams::{Anagram, Anagrams};
use crate::board::board::Board;
use crate::utils::trieguy::TrieTree;
use crate::utils::weighted_group::WeightedGroups;

/// # Agent
/// An agent has a rack, board, and score group.
/// The agent will use the rack, board, and score group to
/// make the best first move possible.
/// ### Functions
/// - `solve()`, `find_best_move()`, `make_move()`, `get_final_score()`
pub struct Agent {
    pub rack: Rack,
    pub board: Board,
    pub score_group: WeightedGroups,
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            rack: Rack::new(),
            board: Board::new(),
            score_group: WeightedGroups::new(),
        }
    }

    /// ### solve()
    /// This function will use the `Rack`, `Board`, `TrieTree` & `score_group`
    /// to solve.
    /// First display the `Rack` and `Board` to user.
    /// Then create a `TrieTree` and generate `Anagrams` from the `Rack`.
    /// After sorting by estimated score, find the best move from the list.
    /// The `Agent` then makes the move, and displays for the user.
    pub fn solve(&mut self) -> () {
        // Display the rack and board
        self.rack.display();
        self.board.display();
        println!("\n...beep boop...solving...");

        // Read in the scrabble dictionary
        let mut tree = TrieTree::new();
        // Generate anagrams
        let mut anagrams = Anagrams::new();
        anagrams.generate("".to_string(), &mut self.rack.clone(), &mut tree);

        // Iterate over the generated anagrams and find the best move
        let mut best_moves: Vec<ScrabbleMove> = vec![];
        for anagram in anagrams {
            best_moves.push(self.find_best_move(&anagram.clone()));
        }
        // Sort best_moves by score
        // best_moves.sort();
        // Make the best move
        self.make_move(&best_moves[0].anagram, best_moves[0].starting_element);
        // Display the board
        self.board.display();
    }

    /// ### find_best_move()
    /// For every letter more than 5 characters, the letters that can land on
    /// the double letter score increases. Meaning, if its 5 characters, the first
    /// or last letter must start on the double letter score in order for the word
    /// to cover the center square. As the word gets longer, we can try more and more
    /// letters on the double letter score.
    /// Given an `Anagram`, determine the best placement for the word and its actual score.
    /// Return a `ScrabbleMove` object (`Anagram` & `starting_element`)
    pub fn find_best_move(&mut self, anagram: &Anagram) -> ScrabbleMove {
        // Set up the word, final_score
        // and a hashmap to easily access the letters and their score
        let word = anagram.word.clone();
        let mut final_score: u32 = 0;
        let mut index_vs_weight_hashmap: HashMap<usize, u32> = HashMap::new();
        for letter in word.chars() {
            let letter_weight = self.score_group.get_score(letter);
            let index = word.chars().position(|c| c == letter).unwrap();
            index_vs_weight_hashmap.insert(index, letter_weight);
        }

        // If the word is less than 5 characters it must start on center square
        if word.len() < 5 {
            // Set the final score
            for letter in word.chars() {
                final_score += self.score_group.get_score(letter);
            }
            self.board.set_score(final_score);
            return ScrabbleMove::new(anagram.clone(), 6);
        }
        // Find the index of the letter with the highest score
        let highest_scoring_letter_index = index_vs_weight_hashmap
            .iter()
            .max_by_key(|x| x.1)
            .unwrap()
            .0;
        // Set the score
        final_score = self.get_final_score(&word, *highest_scoring_letter_index);
        self.board.set_score(final_score);
        // If its 5 characters
        // Check the first and last letter, whichever is higher determines the placement
        // If its the first letter, place starting on element 3; otherwise start on center square
        if word.len() == 5 {
            if *highest_scoring_letter_index == 0 {
                return ScrabbleMove::new(anagram.clone(), 3);
            } else {
                return ScrabbleMove::new(anagram.clone(), 6);
            }
        }
        // If its 6 characters
        else if word.len() == 6 {
            if *highest_scoring_letter_index == 0 {
                return ScrabbleMove::new(anagram.clone(), 3);
            } else if *highest_scoring_letter_index == 1 {
                return ScrabbleMove::new(anagram.clone(), 2);
            } else if *highest_scoring_letter_index == 5 {
                return ScrabbleMove::new(anagram.clone(), 5);
            } else {
                return ScrabbleMove::new(anagram.clone(), 4);
            }
        }
        // Otherwise, its 7 letters
        else {
            if *highest_scoring_letter_index == 0 {
                return ScrabbleMove::new(anagram.clone(), 3);
            } else if *highest_scoring_letter_index == 1 {
                return ScrabbleMove::new(anagram.clone(), 2);
            } else if *highest_scoring_letter_index == 2 {
                return ScrabbleMove::new(anagram.clone(), 1);
            } else if *highest_scoring_letter_index == 5 {
                return ScrabbleMove::new(anagram.clone(), 5);
            } else if *highest_scoring_letter_index == 4 {
                return ScrabbleMove::new(anagram.clone(), 4);
            } else {
                return ScrabbleMove::new(anagram.clone(), 3);
            }
        }
    }

    pub fn get_final_score(&self, word: &str, double_letter_index: usize) -> u32 {
        let mut final_score: u32 = 0;
        let word_chars: Vec<char> = word.chars().collect();
        let double_score = self.score_group.get_score(word_chars[double_letter_index]);
        for letter in word_chars {
            final_score += self.score_group.get_score(letter);
        }
        return final_score + double_score;
    }

    pub fn make_move(&mut self, anagram: &Anagram, mut starting_element: u32) {
        let word = anagram.word.clone();
        for letter in word.chars() {
            self.board.spaces[starting_element as usize] = Some(letter);
            starting_element += 1;
        }
    }
}
