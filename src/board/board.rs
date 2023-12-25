//board/board.rs

// Module: The Board
// Description: The Scrabble board to be solved. This is immutable and always
// the same for every instance of our Agent. In fact, it's not even a whole
// scrabble board because why waste the memory? The starting rack is 7 tiles,
// the first move can only be horizontal, so the board is just the middle 13
// squares of a standard Scrabble Board.

pub struct Board {
    spaces: Vec<Option<char>>,
  }
  
  impl Board {
    pub fn new() -> Board {
      Board {
        spaces: vec![
            Option::None;
            13
        ],
      }
    }

    pub fn display(&self) {
        println!("\n\nBoard:");
        for space in &self.spaces {
            match space {
                Some(c) => print!("{} ", c),
                None => print!("| _ |")
            }
        }
        println!("\n\n--\n");
    }
    pub fn make_move(&mut self, word: &str) {
      println!("{:?}", word);
    }
  }