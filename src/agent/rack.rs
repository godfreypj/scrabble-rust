use rand::seq::SliceRandom;

/// agent/rack.rs
///
/// Module: this is the rack; the letters that the agent uses to solve the puzzle. Every rack is a
/// set of letters, including wildcards (represented by a "_"). Letters are broken up into WeightedGroups
/// that represent the likelyhood of randomly choosing them from a real bag of scrabble letters.
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
    /// Create a new rack with 7 letters.
    /// ## Overview
    /// Every letter exists in a weighted group; which represents
    /// the frequency of those letters in the bag. A bag is instantiated
    /// and shuffled, then a letter chosen. This is repeated 7 times until
    /// we have our rack.
    /// ## Example
    /// ```
    /// let rack = Rack::new();
    /// rack.display();
    /// ```
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
        let mut bag: Vec<char> = Vec::new();
        // Iterate through weighted groups, adding each letter in each group
        // as many times as its weight
        for entry in weighted_groups {
            for _ in 0..entry.weight {
                for letter in &entry.letters {
                    bag.push(*letter);
                }
            }
        }
        for _ in 0..7 {
            // Shuffle the bag
            bag.shuffle(&mut rand::thread_rng());
            // Choose a letter from the first entry
            let letter: char = bag[0];
            // Push it to the rack
            letters.push(letter);
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
