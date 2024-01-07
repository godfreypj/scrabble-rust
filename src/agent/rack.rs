use crate::utils::weighted_group::WeightedGroups;
use rand::seq::SliceRandom;

/// # Rack
/// The rack is the 7 `letters` the agent has to solve the puzzle. Every rack is a
/// set of `letters`, including wildcards (represented by a "_").
/// Letters are broken up into `frequency_groups` that represent the
/// frequency of those letters in a bag of scrabble letters.
/// Functions:
/// - `display()`
pub struct Rack {
    pub letters: Vec<char>,
    pub weighted_groups: WeightedGroups,
}

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
        let weighted_groups = WeightedGroups::new();
        let mut bag: Vec<char> = Vec::new();
        // Iterate through weighted groups, adding each letter in each group
        // as many times as its weight
        for entry in &weighted_groups.frequency_groups {
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

    pub fn display(&self) {
        println!("\n\nRack:");
        for c in &self.letters {
            print!("{}", c);
        }
    }
}
