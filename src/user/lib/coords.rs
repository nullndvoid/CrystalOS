use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use crate::println;


#[derive(Clone, Debug, PartialEq)]
pub enum Line {
    Vertical(i64),
    Horizontal(i64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn touches_line(&self, line: &Line) -> bool {
        match line {
            Line::Vertical(y) => self.y == *y,
            Line::Horizontal(x) => self.x == *x,
        }
    }

    pub fn aligns(&self, other: &Position) -> (bool, bool) {
        (self.x == other.x, self.y == other.y)
    }

    pub fn distance(&self, other: &Position) -> Position {
        Position {
            x: other.x - self.x,
            y: other.y - self.y
        }
    }

    pub fn as_usize(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    pub fn magnitude(&self) -> i64 {
        (self.x.abs() + self.y.abs())
    }

    pub fn nearest(&self, points: &Vec<Position>) -> Position {

        let mut points = points.clone();
        points.sort_by_key(|p| {
            let p = self.distance(p);
            p.x.abs() + p.y.abs()
        });
        points.first().unwrap().to_owned()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    PosY,
    NegY,
    PosX,
    NegX,
    None,
}

impl Direction {
    pub fn rev(&self) -> Direction {
        match self {
            Direction::PosY => Direction::NegY,
            Direction::NegY => Direction::PosY,
            Direction::PosX => Direction::NegX,
            Direction::NegX => Direction::PosX,
            Direction::None => Direction::None,
        }
    }
}