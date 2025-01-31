// SECTION ONE: SETTING UP THE GAME BOARD AND PLAYER
//
// Overview:
// - Define the board's width and height.
// - Build out the player, including what it is and how it will look on the board.
// - Create a game board using a HashMap to represent coordinates (x, y).
// - Place the player at a specific (x, y) position on the game board.
//
// Recap:
// By the end of this section, we will have a game board with a player displayed on it.





// TASK 0: Create the Main Function
// - Create a main function to set up the testing environment. You will add code here to test each task step by step.





// TASK 1: Define Game Board Dimensions
// - Define two constants for the game board's width and height (type: u32).
// - Use `const` to make these values immutable.
// - Set their values to the desired board dimensions, e.g., 10x10 or 20x20.

// To test Task #1, you can print the constants in the main function:
// println!("Board Dimensions: {} x {}", BOARD_WIDTH, BOARD_HEIGHT);

// NOTICE: Once Task #1 is complete, you can comment out this test code.

// TASK 2: Define the Starting Lives for the Player
// - Define a constant for the player's starting lives (type: u8).
// - Use `const` for immutability.
// - Set the value to the desired number of lives (e.g., 5 or 10).

// To test Task #2, you can print the player's starting lives in the main function:
// println!("Player's Starting Lives: {}", STARTING_LIVES);

// NOTICE: Once Task #2 is complete, you can comment out this test code.





// TASK 3: Define a Player Struct
// - Create a struct called `Player` with three fields:
//     - `display_character` (type: char),
//     - `lives` (type: u8),
//     - `current_position` (type: (u8, u8)).
// This struct will represent the player on the board.

// To test Task #3, create an instance of `Player` in the main function:
// let player = Player {
//     display_character: '@',
//     lives: STARTING_LIVES,
//     current_position: (5, 5),
// };
// println!("Player Info: {:?}", player);

// NOTICE: Once Task #3 is complete, you can comment out this test code.





// TASK 4: Implement the Player Struct
// - Implement a `new` method for the `Player` struct.
// - The `new` method should take a parameter `start_position` (a tuple of type (u8, u8)) and return a `Player` instance.

// To test Task #4, you can create a player using the `new` method in the main function:
// let player = Player::new((5, 5));
// println!("Player Created: {:?}", player);

// NOTICE: Once Task #4 is complete, you can comment out this test code.





// TASK #05: We will create a struct called `GameState` that will include a hashmap linking coordinates to characters.
// We will also add a `player` field to the `GameState` struct so we can access player data.

// - Create a struct called `GameState` with two fields: `game_board` and `player`.
// - `game_board` will be a HashMap mapping a tuple of two `u8` values to a `char`. Example:
//   HashMap<(u8, u8), char>.
//   This will link a pair of `u8` values representing coordinates to a character. For example, if you have a 20x20 game board and want to link position (15, 8), the hashmap will return the character stored at that coordinate.
// - `player` will be a reference to a struct `Player`, which stores player data.

// For more information on how to use HashMaps in Rust, check out the official documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

// To test Task #5, you can create a `GameState` instance in the main function:
// let mut game_board = HashMap::new();
// game_board.insert((5, 5), '@');
// let player = Player::new((5, 5));
// let game_state = GameState { game_board, player: &player };
// println!("Game State: {:?}", game_state);

// NOTICE: Once Task #5 is complete, you can comment out this test code.
