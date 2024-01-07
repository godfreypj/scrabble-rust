mod agent {
    pub(crate) mod agent;
    pub(crate) mod rack;
    pub(crate) mod scrabble_move;
}

mod anagrams {
    pub(crate) mod anagrams;
}

mod board {
    pub(crate) mod board;
}

mod utils {
    pub(crate) mod trieguy;
    pub(crate) mod weighted_group;
}

use crate::agent::agent::Agent;

fn main() {
    let mut agent = Agent::new();
    println!("\n\n===========SCRABBLE===========\n\n");
    agent.solve();
}
