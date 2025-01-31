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

// TASK #01: Define Game Board Dimensions
// - Use two constants to represent the board's width and height (type: u32).
// - Since the board dimensions are fixed, use `const` as it ensures the variables remain immutable.
// - Set their values to your desired board width and height. For example: 10x10, 20x20, etc.

// TASK #02: Define the starting lives for the player
// - Use a constant variable to represent the player's starting lives (type: u8).
// - Since the player's starting lives will never change, we use `const`, which ensures the variable remains immutable.
// - Set its value to your desired number of lives. For example: 5, 10, etc.


// TASK #03: Define a Player struct that holds the player's display character, lives, and current position.
// - Create a struct called `Player`.
// - Add three fields: `display_character`, `lives`, and `current_position`.
// - `display_character` will have the type `char`.
// - `lives` will have the type `u8`.
// - `current_position` will be a tuple of type `(u8, u8)`.

// NOTICE: You have not yet learned what a tuple is. Here is a brief explanation.
// A tuple is a fixed-size collection of values of different types.
// Example:
//let my_tuple = (42, 54);

// Accessing elements inside the touple:
//let first = my_tuple.0; // 42

// More info: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type

// TASK #04: Define an implementation for the struct Player.
// - Create an `impl` block for the `Player` struct.
// - Define a method called `new` that takes the parameter `start_position`, which is a tuple of two `u8`s.
// - The method must return `Self`.
// - Now, define the necessary fields for the struct.
