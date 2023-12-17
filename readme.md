# Scrabble written in Rust

## This program will take a 'rack' and return the best first move in Scrabble.

<hr>

### Quick Start:

- rust
- Navigate to the proper directory and run `cargo run main.rs`

### Overview:

The goal of this app is to develop an `agent` to find the optimal **initial** move of a Scrabble game, given the following inputs:

- An empty Scrabble board.
  - Assume the agent knows how the premium squares work & locations.
- The complete SOWPODS word list, containing all 267,751 legal words.
- A collection of 7 tiles in the agent’s rack.
  - At most 2 may be wildcard tiles.
  - `rack` is created the same way a person would select their letters; from a bag that represents the real 
  frequency of scrabble letters.

### Board:

- The board is 13 `squares` in order to minimize complexity; the rules state the first move must cover the middle square and with a max word size of 7 `tiles` the board only needs to be 13 elements.
- Included is a `placeWord` function to place the word, and `display` to show the state of the board

### Agent:

- The Agent is responsible for arriving at the goal. It instantiates the `board` the `rack` and also contains a `display` function for displaying the rack.
- The `solve` functions runs through the `utils` module and solves the board:
  - `generate_anagrams` instantiates the `TrieGuy` which is a prefix-tree that holds the official scrabble dictionary. In this data structure all words with common roots share common nodes. This significantly reduces the search time to find a legal word.
  - As anagrams are generated the function uses `estimateScore` to compare the estimated maximum score of the rack to the estimated score of a given anagram; if the word is at least 40% of `maxScore` it is added to the `anagrams`. This eliminates the need to actually calculate the score for each potential word & significantly reduces the `anagrams` list.
  - The final `anagrams` list is then iterated over and the `findBestMove` function decides where to place it using these rules:
    - If the word is less than 4 characters, it must start on the center tile
    - Put the highest scoring letter on the double letter tile such that a legal move is produced
  - The scores resulting from `findBestMove` are compared and the highest scoring move is chosen
  - Results are displayed to the user
  