// agent/rack.rs
//
// Module: this is the rack; the letters that the agent uses to solve the puzzle. Every rack is a random
// set of letters, including wildcards (represented by a "_").

use rand::seq::SliceRandom;

pub struct Rack {
    pub letters: Vec<char>,
}

struct WeightedGroup {
    letters: Vec<char>,
    weight: u32,
}

impl WeightedGroup {
    fn new(letters: Vec<char>, weight: u32) -> Self {
        WeightedGroup { letters, weight }
    }
}

impl Rack {
    pub fn new() -> Rack {
        let mut letters: Vec<char> = Vec::new();
        let weighted_groups = vec![
            WeightedGroup::new(vec!['J', 'K', 'U', 'X', 'Z'], 1),
            WeightedGroup::new(vec!['B', 'C', 'F', 'H', 'M', 'P', 'V', 'W', 'Y', '_'], 2),
            WeightedGroup::new(vec!['G'], 3),
            WeightedGroup::new(vec!['D', 'L', 'S', 'U'], 4),
            WeightedGroup::new(vec!['N', 'R', 'T'], 6),
            WeightedGroup::new(vec!['O'], 8),
            WeightedGroup::new(vec!['A', 'I'], 9),
            WeightedGroup::new(vec!['E'], 12)
        ];
        for _ in 0..7 {
            // Read the weighted groups into entries, randomly
            let mut entries = weighted_groups.iter().collect::<Vec<_>>();
            entries.shuffle(&mut rand::thread_rng());
            // Choose a letter from the first entry
            let letter = entries[0].letters.choose(&mut rand::thread_rng()).unwrap();
            // Push it to the rack
            letters.push(*letter);
        }
        Rack { letters }
    }

    pub fn display(&self) {
        println!("\n\nRack:");
        for c in &self.letters {
            print!("{}", c);
        }
    }
}
