mod agent;
mod board;
use agent::agent::Agent;

use crate::agent::utils::legalmove::Move;

fn main() {
    let mut agent = Agent::new();
    println!("\n\n===========SCRABBLE===========\n\n");
    let best_move: Move = agent.solve();
    agent.board.make_move(best_move.anagram, best_move.starting_element);
    agent.rack.display();
    agent.board.display();
}