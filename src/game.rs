use crate::{
    board::Board,
    components::{ActivePlayer, PlaceResult, Score, TileType},
};
// use std::io::Stdin;
use std::{
    io,
    io::{stdin, Write},
};

/// Game struct manages the course of the game.
/// Contains the public methods to create and start a new game
pub struct Game {
    board: Board,
    score: Score,
    active_player: ActivePlayer,
}

impl Game {
    /// Creates a new Game instance.
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            score: Score::new(),
            active_player: ActivePlayer::Player1,
        }
    }

    /// Starts a single round.
    pub fn start(&mut self) {
        loop {
            print!("{}", self.board);
            print!("Row number: ");
            io::stdout()
                .lock()
                .flush()
                .ok()
                .expect("Could not flush stdout");

            let row = self.get_row();
            if self.board.place_tile(row, self.active_player.to_tiletype()) != PlaceResult::Success
            {
                println!("Unable to place piece at row {}", row);
                continue;
            }

            match self.board.check4() {
                TileType::Player1 => {
                    self.score.p1 += 1;
                    println!("{} wins the match", self.active_player);
                    self.active_player.swap();
                    println!("{}", self.board);
                    break;
                }
                TileType::Player2 => {
                    self.score.p2 += 1;
                    println!("{} wins the match", self.active_player);
                    self.active_player.swap();
                    println!("{}", self.board);
                    break;
                }
                _ => {
                    self.active_player.swap();
                }
            }
        }
        println!(
            "Player 1 points: {}\nPlayer 2 points: {}",
            self.score.p1, self.score.p2
        );
    }

    /// Starts a new round after the end of every round, until the player decides to exit.
    pub fn start_loop(&mut self) {
        loop {
            self.start();
            if !self.ask_restart() {
                break;
            };
            self.board.clear();
        }
    }

    fn ask_restart(&self) -> bool {
        println!("Do you want to play again? Y/n");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                if buffer == "\n" || buffer == "\r\n" {
                    return true;
                } else {
                    match buffer.trim().to_lowercase().as_str() {
                        "y" | "yes" => return true,
                        "n" | "no" => return false,
                        _ => {
                            println!("Invalid input. Must be y, yes, n, no");
                            return self.ask_restart();
                        }
                    }
                }
            }
            Err(e) => println!("{}", e),
        }

        false
    }

    fn get_row(&mut self) -> usize {
        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<usize>() {
                Ok(i) => {
                    if i > 0 && i < 8 {
                        return i;
                    } else {
                        println!("The number must be between 1 and 7");
                        self.get_row()
                    }
                }
                Err(_) => {
                    println!("The input must be a number between 1 and 7");
                    self.get_row()
                }
            },
            Err(e) => {
                println!("Invalid input: {}", e);
                self.get_row()
            }
        }
    }
}
