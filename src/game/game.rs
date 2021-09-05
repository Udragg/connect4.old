use crate::{
    ai::ai::Ai,
    game::{
        board::Board,
        components::{ActivePlayer, PlaceResult, Score, TileType},
    },
};
// use std::io::Stdin;
use std::io::{stdin, stdout, Write};

/// Game struct manages the course of the game.
/// Contains the public methods to create and start a new game
pub struct Game {
    board: Board,
    score: Score,
    active_player: ActivePlayer,

    ai: Option<Ai>,
}

impl Game {
    /// Creates a new Game instance.
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            score: Score::new(),
            active_player: ActivePlayer::Player1,
            ai: None,
        }
    }

    /// Adds an Ai instance to the game
    // pub fn add_ai(&mut self) {
    //     self.ai = Some(Ai::new(Board::default()));
    // }

    fn get_input(&self) -> String {
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => buf.trim().to_string(),
            Err(e) => panic!("error reading stdin: {}", e),
        }
    }

    fn ask_restart(&self) -> bool {
        println!("Do you want to play again? Y/n");
        let input = self.get_input();

        match input.as_str() {
            "y" | "yes" | "" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Interpreting vague answer as YES.");
                return true;
            }
        }
    }

    fn get_row(&mut self) -> usize {
        loop {
            print!("Row number: ");
            stdout()
                .lock()
                .flush()
                .ok()
                .expect("Could not flush stdout");
            let row_str = self.get_input();

            match row_str.parse::<usize>() {
                Ok(i) => {
                    if i > 0 && i < 8 {
                        return i;
                    } else {
                        println!("The number must be between 1 and 7");
                    }
                }
                Err(_) => println!("The inpupt must be a number between 1 and 7"),
            }
        }
    }

    /// Starts a single round.
    pub fn start(&mut self) {
        loop {
            print!("{}", self.board);
            println!("{}'s turn", self.active_player);

            let row = self.get_row();
            if self.board.place_tile(row, self.active_player.to_tiletype()) != PlaceResult::Success
            {
                println!("Unable to place piece at row {}", row);
                continue;
            }

            match self.board.check4() {
                TileType::Player1 => {
                    self.score.p1 += 1;
                    break;
                }
                TileType::Player2 => {
                    self.score.p2 += 1;
                    break;
                }
                _ => {
                    self.active_player.swap();
                }
            }
        }
        print!("{}", self.board);
        println!("{} wins the match", self.active_player);
        self.active_player.swap();

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
}
