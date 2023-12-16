// agent/agent.rs
//
// Module: Agent module. Every agent instantiates a Rack, Board & then solves.

use rack::Rack;
use crate::board::board::Board;

use super::rack;

pub struct Agent {
    rack: Rack,
    pub board: Board,
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            rack: Rack::new(),
            board: Board::new(),
        }
    }

    pub fn solve(&self) {
        self.rack.display();
        self.board.display();
        println!("\n...beep boop...solving...");
    }
}
