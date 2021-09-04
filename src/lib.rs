#![deny(warnings)]
#![warn(unused)]
#![warn(non_camel_case_types)]
#![warn(non_snake_case)]

#[warn(missing_docs)]
/// Module for the Board struct
mod board;

/// Module containing miscellaneous components used within the board and game modules
mod components;

// pub mod error;

#[warn(missing_docs)]
/// Module for the Game struct
pub mod game;

/// Module containing tests
mod tests;
