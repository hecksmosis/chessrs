use std::error::Error;
use chessrs::main_loop;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Add command line arguments

    // Runs the game
    main_loop().map(|_| ())
}
