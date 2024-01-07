pub struct WeightedGroup {
    pub letters: Vec<char>,
    pub weight: u32,
}

/// ## WeightedGroup
/// ### Fields
/// - letters: Vec<char>
/// - weight: u32
impl WeightedGroup {
    pub fn new(letters: Vec<char>, weight: u32) -> Self {
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
    pub frequency_groups: Vec<WeightedGroup>,
    pub score_groups: Vec<WeightedGroup>,
}

impl Clone for WeightedGroups {
    fn clone(&self) -> Self {
        Self {
            frequency_groups: self.frequency_groups.clone(),
            score_groups: self.score_groups.clone(),
        }
    }
}

impl Iterator for WeightedGroups {
    type Item = WeightedGroup;
    fn next(&mut self) -> Option<Self::Item> {
        self.frequency_groups.pop(); // Remove and return the last weighted group
        self.score_groups.pop() // Remove and return the last weighted group
    }
}

/// ## WeigthedGroups
/// ### Fields
/// - frequency_groups: Vec<WeightedGroup>
/// - score_groups: Vec<WeightedGroup>
/// Two groups are instantiated, frequency - representing the letters in a scrabble bag
/// and scores, representing the scores
impl WeightedGroups {
    pub fn new() -> Self {
        let frequency_groups = vec![
            WeightedGroup::new(vec!['J', 'K', 'U', 'X', 'Z'], 1),
            WeightedGroup::new(vec!['B', 'C', 'F', 'H', 'M', 'P', 'V', 'W', 'Y', '_'], 2),
            WeightedGroup::new(vec!['G'], 3),
            WeightedGroup::new(vec!['D', 'L', 'S', 'U'], 4),
            WeightedGroup::new(vec!['N', 'R', 'T'], 6),
            WeightedGroup::new(vec!['O'], 8),
            WeightedGroup::new(vec!['A', 'I'], 9),
            WeightedGroup::new(vec!['E'], 12),
        ];
        let score_groups = vec![
            WeightedGroup::new(vec!['A', 'E', 'I', 'L', 'N', 'O', 'R', 'S', 'T', 'U'], 1),
            WeightedGroup::new(vec!['D', 'G'], 2),
            WeightedGroup::new(vec!['B', 'C', 'M', 'P'], 3),
            WeightedGroup::new(vec!['F', 'H', 'V', 'W', 'Y'], 4),
            WeightedGroup::new(vec!['K'], 5),
            WeightedGroup::new(vec!['J', 'X'], 8),
            WeightedGroup::new(vec!['Q', 'Z'], 10),
            WeightedGroup::new(vec!['E'], 12),
        ];
        WeightedGroups {
            frequency_groups,
            score_groups,
        }
    }
    pub fn get_score(&self, letter: char) -> u32 {
        for group in &self.score_groups {
            if group.letters.contains(&letter) {
                return group.weight;
            }
        }
        0
    }
}
