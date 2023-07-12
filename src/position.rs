use std::{ fmt::Display, ops::{ Add, Sub } };

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = ((self.x as u8) + ('a' as u8)) as char;
        let y = ((self.y as u8) + ('1' as u8)) as char;

        write!(f, "{}{}", x, y)
    }
}

impl Into<(isize, isize)> for Position {
    fn into(self) -> (isize, isize) {
        (self.x as isize, self.y as isize)
    }
}

impl From<(usize, usize)> for Position {
    fn from(tuple: (usize, usize)) -> Self {
        Position {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<(isize, isize)> for Position {
    fn from(tuple: (isize, isize)) -> Self {
        Position {
            x: tuple.0 as usize,
            y: tuple.1 as usize,
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from(tuple: (i32, i32)) -> Self {
        Position {
            x: tuple.0 as usize,
            y: tuple.1 as usize,
        }
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<(isize, isize)> for Position {
    type Output = Position;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Position {
            x: ((self.x as isize) + rhs.0) as usize,
            y: ((self.y as isize) + rhs.1) as usize,
        }
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Position;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Position {
            x: ((self.x as i32) + rhs.0) as usize,
            y: ((self.y as i32) + rhs.1) as usize,
        }
    }
}

impl Add<(i8, i8)> for Position {
    type Output = Position;

    fn add(self, rhs: (i8, i8)) -> Self::Output {
        Position {
            x: ((self.x as i8) + rhs.0) as usize,
            y: ((self.y as i8) + rhs.1) as usize,
        }
    }
}

impl Sub<(i8, i8)> for Position {
    type Output = Position;

    fn sub(self, rhs: (i8, i8)) -> Self::Output {
        Position {
            x: ((self.x as i8) - rhs.0) as usize,
            y: ((self.y as i8) - rhs.1) as usize,
        }
    }
}

impl Position {
    pub fn with_x(&self, x: usize) -> Self {
        Position { x, y: self.y }
    }

    pub fn with_y(&self, y: usize) -> Self {
        Position { x: self.x, y }
    }

    pub fn from_byte(byte: u8) -> Self {
        let x = (byte & 0b111000) as usize;
        let y = (byte / 0b111) as usize;

        Position { x, y }
    }

    pub fn to_byte(&self) -> u8 {
        ((self.x << 3) | self.y) as u8
    }
}
