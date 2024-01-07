use crate::anagrams::anagrams::Anagram;

/// ## ScrabbleMove
/// A simple struct that holds the result of
/// the find_best_move() function
pub struct ScrabbleMove {
    pub anagram: Anagram,
    pub starting_element: u32,
}

impl ScrabbleMove {
    pub fn new(anagram: Anagram, starting_element: u32) -> Self {
        ScrabbleMove {
            anagram,
            starting_element,
        }
    }
}
