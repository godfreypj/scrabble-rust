use super::anagram::Anagram;

// ## Move
// A simple struct that holds the result of
// the find_best_move() function
pub struct Move {
    pub anagram: &Anagram,
    pub starting_element: u32,
}

impl Move {
    pub fn new(anagram: &Anagram, starting_element: u32) -> Self {
        Move {
            anagram,
            starting_element,
        }
    }
}