mod agent;
mod board;
use agent::agent::Agent;
use agent::trieguy::TrieTree;

fn main() {
    let agent = Agent::new();
    let mut tree = TrieTree::new();
    let result = tree.search("RUST");
    println!("{:?}", result);
    agent.solve();
}
