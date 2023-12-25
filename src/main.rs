mod agent;
mod board;
use agent::agent::Agent;

use crate::agent::utils::anagram::Move;

fn main() {
    let agent = Agent::new();
    println!("\n\n===========SCRABBLE===========\n\n");
    let best_move: Move = agent.solve();
    println!(
        "Best move: {:?}\nStarting Square: {:?}",
        best_move.word, best_move.starting_element
    )
}