pub use eval::*;
pub use game::{checks::*, default::*, *};
pub use input::*;
pub use moves::*;
pub use piece::*;
pub use position::*;
pub use std::{
    array,
    convert::TryInto,
    error::Error,
    fmt::{Debug, Display, Formatter, Result as fmtResult},
    io::{self, Write},
};

mod eval;
mod game;
mod input;
mod moves;
mod piece;
mod position;
#[cfg(test)]
mod tests;

pub fn main_loop() -> ! {
    let mut game = Game::default();
    println!("{}", game);

    loop {
        print!(
            "Enter move for {} (id: {}): ",
            if game.turn == 0 { "white" } else { "black" },
            game.turn
        );
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!(
            "{}",
            match handle_input(&mut game, &mut input) {
                Ok(_) => "Move successful".to_string(),
                Err(e) => e.to_string(),
            }
        );
    }
}

fn handle_input(game: &mut Game, input: &mut String) -> Result<(), Box<dyn Error>> {
    let coords = match Input::build(input.trim()) {
        Ok(coords) => coords,
        Err(e) => {
            return Err(e.into());
        }
    };

    match game.make_move(coords) {
        Ok(_) => {
            dbg!(&game.check);
            println!("{}", game);
            println!("{}", Eval::from(&game));
            let _ = game.check_win();
        }
        Err(e) => {
            return Err(e.into());
        }
    };

    Ok(())
}
