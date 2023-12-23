use rand::seq::SliceRandom;

/// agent/rack.rs
///
/// Module: this is the rack; the letters that the agent uses to solve the puzzle. Every rack is a
/// set of letters, including wildcards (represented by a "_"). Letters are broken up into WeightedGroups
/// that represent the likelyhood of randomly choosing them from a real bag of scrabble letters.
pub struct Rack {
    pub letters: Vec<char>,
    pub weighted_groups: Vec<WeightedGroup>,
}

pub struct WeightedGroup {
    pub letters: Vec<char>,
    pub weight: u32,
}

impl WeightedGroup {
    fn new(letters: Vec<char>, weight: u32) -> Self {
        WeightedGroup { letters, weight }
    }
}

impl Clone for WeightedGroup {
    fn clone(&self) -> Self {
        Self {
            letters: self.letters.clone(),
            weight: self.weight,
        }
    }
}

pub struct WeightedGroups {
    weighted_groups: Vec<WeightedGroup>
}

impl Clone for WeightedGroups {
    fn clone(&self) -> Self {
        Self {
            weighted_groups: self.weighted_groups.clone()
        }
    }
}

impl WeightedGroups {
    fn new() -> Self {
        let weighted_groups = vec![
            WeightedGroup::new(vec!['J', 'K', 'U', 'X', 'Z'], 1),
            WeightedGroup::new(vec!['B', 'C', 'F', 'H', 'M', 'P', 'V', 'W', 'Y', '_'], 2),
            WeightedGroup::new(vec!['G'], 3),
            WeightedGroup::new(vec!['D', 'L', 'S', 'U'], 4),
            WeightedGroup::new(vec!['N', 'R', 'T'], 6),
            WeightedGroup::new(vec!['O'], 8),
            WeightedGroup::new(vec!['A', 'I'], 9),
            WeightedGroup::new(vec!['E'], 12),
        ];
        WeightedGroups { weighted_groups }
    }
}

// ... your existing code

impl Clone for Rack {
    fn clone(&self) -> Self {
        Self {
            letters: self.letters.clone(),
            weighted_groups: self.weighted_groups.clone(),
        }
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
        let weighted_groups = WeightedGroups::new().weighted_groups;
        let mut bag: Vec<char> = Vec::new();
        // Iterate through weighted groups, adding each letter in each group
        // as many times as its weight
        for entry in &weighted_groups {
            for _ in 0..entry.weight {
                for letter in &entry.letters {
                    bag.push(*letter);
                }
            }
        }
        for _ in 0..7 {
            // Shuffle the bag
            bag.shuffle(&mut rand::thread_rng());
            // Iterate over the shuffled bag
            for ltr in &bag {
                // If the letter is a wildcard & letters already contains 2
                // pick the next letter
                if *ltr == '_' && letters.iter().filter(|&x| *x == '_').count() > 1 {
                    continue;
                } else {
                    letters.push(*ltr);
                    break;
                }
            }
        }
        Rack {
            letters,
            weighted_groups,
        }
    }

    /// ### Overview
    /// This function iterates over the rack, finding each
    /// letter in its weighted group. The letter with the
    /// highest score is doubled, the rest are added.
    pub fn max_score(&self) -> u32 {
        let mut score: u32 = 0;
        let mut max_score: u32 = 0;
        for letter in &self.letters {
            for group in &self.weighted_groups {
                if group.letters.contains(&letter) {
                    if max_score < group.weight {
                        max_score = group.weight;
                    }
                    score += group.weight;
                }
            }
        }
        score += max_score;
        score
    }
    pub fn display(&self) {
        println!("\n\nRack:");
        for c in &self.letters {
            print!("{}", c);
        }
    }
}
