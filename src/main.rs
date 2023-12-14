mod agent;
mod board;
use agent::agent::Agent;

fn main() {
    let agent = Agent::new();
    agent.solve();
}
