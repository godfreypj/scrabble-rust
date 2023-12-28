/// # Scores
/// ### This module contains the same type of object as a WeightedGroup
/// ### each letter is in a ScoreGroup, with iterator and clone implemented

/// A group of letters with a score
pub struct ScoreGroup {
    pub letters: Vec<char>,
    pub score: u32,
}

impl ScoreGroup {
    fn new(letters: Vec<char>, score: u32) -> Self {
        ScoreGroup { letters, score }
    }
}

impl Clone for ScoreGroup {
    fn clone(&self) -> Self {
        Self {
            letters: self.letters.clone(),
            score: self.score,
        }
    }
}

pub struct ScoreGroups {
    pub score_groups: Vec<ScoreGroup>
}

impl Clone for ScoreGroups {
    fn clone(&self) -> Self {
        Self {
            score_groups: self.score_groups.clone()
        }
    }
}

impl Iterator for ScoreGroups {
    type Item = ScoreGroup;
    fn next(&mut self) -> Option<Self::Item> {
        self.score_groups.pop() // Remove and return the last weighted group
    }
}

impl ScoreGroups {
    pub fn new() -> Self {
        let score_groups = vec![
            ScoreGroup::new(vec!['A', 'E', 'I', 'L', 'N', 'O', 'R', 'S', 'T', 'U'], 1),
            ScoreGroup::new(vec!['D', 'G'], 2),
            ScoreGroup::new(vec!['B', 'C', 'M', 'P'], 3),
            ScoreGroup::new(vec![ 'F', 'H', 'V', 'W', 'Y'], 4),
            ScoreGroup::new(vec!['K'], 5),
            ScoreGroup::new(vec!['J', 'X'], 8),
            ScoreGroup::new(vec!['Q', 'Z'], 10),
            ScoreGroup::new(vec!['E'], 12),
        ];
        ScoreGroups { score_groups }
    }
    pub fn get_score(&self, letter: char) -> u32 {
        for group in &self.score_groups {
            if group.letters.contains(&letter) {
                return group.score;
            }
        }
        0
    }
}