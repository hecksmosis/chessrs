pub use coords::*;
pub use game::{checks::*, *};
pub use piece::*;
pub use std::{
    array,
    convert::TryInto,
    fmt::{Debug, Display, Formatter, Result as fmtResult},
    io::{self, Write},
};

mod coords;
mod game;
mod piece;

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

        let coords = match Coords::build(input.trim()) {
            Ok(coords) => coords,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        match game.make_move(coords) {
            Ok(_) => {
                game.check = game.is_king_in_check();
                game.turn ^= 1;
                println!("{}", game);
                println!("{}", game.check);
            }
            Err(e) => {
                println!("{}, try again", e);
                continue;
            }
        };
    }
}
