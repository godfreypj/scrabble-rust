use rack::Rack;
use crate::{board::board::Board, agent::utils::{anagram::Anagrams, trieguy::TrieTree}};

use super::{rack, utils::anagram::Move};

/// agent/agent.rs
///
/// Module: Agent module. Every agent instantiates a Rack, Board & then solves.
pub struct Agent {
    pub rack: Rack,
    pub board: Board,
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            rack: Rack::new(),
            board: Board::new(),
        }
    }

    pub fn solve(&self) -> Move {
        // Display the rack and board
        self.rack.display();
        self.board.display();
        println!("\n...beep boop...solving...");
        // Read in the scrabble dictionary
        let mut tree = TrieTree::new();
        // Generate anagrams
        let mut anagrams = Anagrams::new(self.rack.clone());
        anagrams.generate("".to_string(), &mut self.rack.clone(), &mut tree);
        // Get the best move
        let best_move = anagrams.find_best_move();
        // Display the best move
        return best_move;


    }
}
