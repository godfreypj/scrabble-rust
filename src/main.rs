mod agent;
mod board;
use agent::agent::Agent;
use agent::utils::trieguy::TrieTree;

use crate::agent::utils::anagram::Anagrams;

fn main() {
    let agent = Agent::new();
    let mut rack = agent.rack.clone();
    let mut tree = TrieTree::new();
    let mut anagrams = Anagrams::new(rack.clone());
    anagrams.generate("".to_string(), &mut rack, &mut tree);
    for word in anagrams {
        println!("{:?}", word);
    }
    agent.solve();
}