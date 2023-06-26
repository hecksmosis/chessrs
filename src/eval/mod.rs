use crate::*;

pub struct Eval {
    pub value: i32,
}

pub trait Value {
    fn value(&self) -> i32;
}

impl Display for Eval {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "Evaluation: {}", self.value)
    }
}

impl From<&&mut Game> for Eval {
    fn from(game: &&mut Game) -> Eval {
        let value = game
            .get_board()
            .iter()
            .flatten()
            .map(|piece| piece.value())
            .sum();
        Eval { value }
    }
}
