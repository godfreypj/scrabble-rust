/// # Board
/// The board represents the middle row of the Scrabble Board. It's a vector of "spaces"
/// and the final score, which the agent sets.
/// Functions:
/// - `set_score()`, `display()`
pub struct Board {
    pub spaces: Vec<Option<char>>,
    pub score: u32,
}

impl Board {
    pub fn new() -> Board {
        Board {
            spaces: vec![Option::None; 13],
            score: 0,
        }
    }

    pub fn set_score(&mut self, score: u32) {
        self.score = score;
    }

    pub fn display(&self) {
        println!("\n\nBoard:");
        for space in &self.spaces {
            match space {
                Some(c) => print!("| {} |", c),
                None => print!("| _ |"),
            }
        }
        println!("\n\nScore: {:?}", self.score);
        println!("\n\n--\n");
    }
}
