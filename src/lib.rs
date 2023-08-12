pub use eval::*;
pub use game::{ checks::*, default::*, *, GameResult::* };
pub use input::*;
pub use moves::*;
pub use piece::*;
pub use position::*;
pub use std::{
    array,
    convert::TryInto,
    error::Error,
    fmt::{ Debug, Display, Formatter, Result as fmtResult },
    io::{ self, Write },
};

mod eval;
mod game;
mod input;
mod moves;
mod piece;
mod position;
#[cfg(test)]
mod tests;

pub fn main_loop() -> Result<bool, Box<dyn Error>> {
    let mut game = Game::default();
    println!("{}", game);

    loop {
        print!(
            "Enter move for {} (id: {}): ",
            if game.turn == 0 {
                "white"
            } else {
                "black"
            },
            game.turn
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let out = match handle_input(&mut game, &mut input) {
            Win(winner) => format!("{} wins", if winner == 0 { "white" } else { "black" }),
            Draw => "Game drawn".to_string(),
            InProgress(r) =>
                match r {
                    Ok(_) => "Move succesful".to_string(),
                    Err(e) => {
                        return Err(e);
                    }
                }
        };

        println!("{}", out);
    }
}

fn handle_input(game: &mut Game, input: &mut String) -> GameResult {
    let input = match input!(input.trim()) {
        Ok(input) => input,
        Err(e) => {
            return InProgress(Err(e.into()));
        }
    };

    match game.make_pmove(input) {
        Ok(_) => {
            dbg!(&game.check);
            println!("{}", game);
            println!("{}", Eval::from(&game));
            if game.check_win() {
                return Win(game.turn);
            } else if game.check_draw() {
                return Draw;
            }
        }
        Err(e) => {
            return InProgress(Err(e.into()));
        }
    }

    InProgress(Ok(()))
}
