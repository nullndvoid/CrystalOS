use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use libm::sqrt;
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

/// a point represented with x and y coordinates.
impl Position {

    /// checks if the point is on a given line
    pub fn touches_line(&self, line: &Line) -> bool {
        match line {
            Line::Vertical(y) => self.y as i64 == *y,
            Line::Horizontal(x) => self.x as i64 == *x,
        }
    }

    /// checks if two points are equal and returns the x and y equality result for each
    pub fn aligns(&self, other: &Position) -> (bool, bool) {
        (self.x == other.x, self.y == other.y)
    }

    /// calculates x + y distance between two points
    pub fn get_offset(&self, other: &Position) -> Position {
        Position {
            x: other.x - self.x,
            y: other.y - self.y
        }
    }

    pub fn diagonal_distance(&self, other: &Position) -> i64 {
        sqrt((self.x - other.x).pow(2) as f64 + (self.y - other.y).pow(2) as f64) as i64
    }

    pub fn as_usize(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    pub fn magnitude(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    pub fn nearest(&self, points: &Vec<Position>) -> Position {

        let mut points = points.clone();
        points.sort_by_key(|p| {
            let p = self.get_offset(p);
            p.x.abs() + p.y.abs()
        });
        points.first().unwrap().to_owned()
    }


    pub fn rotated_aroundte(&self, angle: Direction, p: Position) -> Position { // rotates by an angle around a point

        // gets coords relative to point to rotate around
        let mut p_offset = self.get_offset(&p);

        p_offset = match angle { // default angle is posy = 0 degrees and negy = 180
            Direction::Degrees0 => Position {
                x: p_offset.x,
                y: p_offset.y,
            },
            Direction::Degrees90 => Position {
                x: -p_offset.y,
                y: p_offset.x,
            },
            Direction::Degrees180 => Position {
                x: -p_offset.x,
                y: -p_offset.y,
            },
            Direction::Degrees270 => Position {
                x: p_offset.y,
                y: -p_offset.x,
            },
            Direction::None => panic!("direction should never be none in this application"),
        };

        return p_offset + p;
    }

    pub fn rotate(&self, angle: Direction) -> Position { // rotates by an angle around origin
        match angle { // default angle is posy = 0 degrees and negy = 180
            Direction::Degrees0 => Position {
                x: self.x,
                y: self.y,
            },
            Direction::Degrees90 => Position {
                x: -self.y,
                y: self.x,
            },
            Direction::Degrees180 => Position {
                x: -self.x,
                y: -self.y,
            },
            Direction::Degrees270 => Position {
                x: self.y,
                y: -self.x,
            },
            Direction::None => panic!("direction should never be none in this application"),
        }
    }
    pub fn real(self) -> PositionReal {
        PositionReal {
            x: self.x as f64,
            y: self.y as f64,
        }
    }

    pub fn zero() -> Position {
        Position {
            x: 0,
            y: 0,
        }
    }
}

impl core::ops::Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// can be expressed as degrees relative to north (where north faces towards the top of the screen)
/// none variant used for when the value is missing / no value is decided.
#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Degrees0,
    Degrees180,
    Degrees90,
    Degrees270,
    None,
}

impl Direction {
    pub fn rev(&self) -> Direction {
        match self {
            Direction::Degrees0 => Direction::Degrees180,
            Direction::Degrees180 => Direction::Degrees0,
            Direction::Degrees90 => Direction::Degrees270,
            Direction::Degrees270 => Direction::Degrees90,
            Direction::None => Direction::None,
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct PositionReal {
    pub x: f64,
    pub y: f64,
}

/// a point represented with x and y coordinates.
impl PositionReal {

    /// checks if the point is on a given line
    pub fn touches_line(&self, line: &Line) -> bool {
        match line {
            Line::Vertical(y) => self.y as i64 == *y,
            Line::Horizontal(x) => self.x as i64 == *x,
        }
    }

    /// checks if two points are equal and returns the x and y equality result for each
    pub fn aligns(&self, other: &PositionReal) -> (bool, bool) {
        (self.x == other.x, self.y == other.y)
    }

    /// calculates x + y distance between two points
    pub fn get_offset(&self, other: &PositionReal) -> PositionReal {
        PositionReal {
            x: other.x - self.x,
            y: other.y - self.y
        }
    }

    pub fn diagonal_distance(&self, other: &PositionReal) -> f64 {
        sqrt((self.x - other.x)*(self.x - other.x) + (self.y - other.y)* (self.y - other.y) as f64)
    }

    pub fn as_usize(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    pub fn magnitude(&self) -> i64 {
        let absx = if self.x >= 0.0 { self.x } else { -self.x };
        let absy = if self.y >= 0.0 { self.y } else { -self.y };
        (absx + absy) as i64

    }

    pub fn nearest(&self, points: &Vec<PositionReal>) -> PositionReal {

        let mut points = points.clone();
        points.sort_by_key(|p| {
            let p = self.get_offset(p);
            let absx = if p.x >= 0.0 { p.x } else { -p.x };
            let absy = if p.y >= 0.0 { p.y } else { -p.y };
            (absx + absy) as i64
        });
        points.first().unwrap().to_owned()
    }


    pub fn rotated_around(&self, angle: Direction, p: PositionReal) -> PositionReal { // rotates by an angle around a point

        // gets coords relative to point to rotate around
        let mut p_offset = self.get_offset(&p);

        p_offset = match angle { // default angle is posy = 0 degrees and negy = 180
            Direction::Degrees0 => PositionReal {
                x: p_offset.x,
                y: p_offset.y,
            },
            Direction::Degrees90 => PositionReal {
                x: -p_offset.y,
                y: p_offset.x,
            },
            Direction::Degrees180 => PositionReal {
                x: -p_offset.x,
                y: -p_offset.y,
            },
            Direction::Degrees270 => PositionReal {
                x: p_offset.y,
                y: -p_offset.x,
            },
            Direction::None => panic!("direction should never be none in this application"),
        };

        return p_offset + p;
    }

    pub fn rotate(&self, angle: Direction) -> PositionReal { // rotates by an angle around origin
        match angle { // default angle is posy = 0 degrees and negy = 180
            Direction::Degrees0 => PositionReal {
                x: self.x,
                y: self.y,
            },
            Direction::Degrees90 => PositionReal {
                x: -self.y,
                y: self.x,
            },
            Direction::Degrees180 => PositionReal {
                x: -self.x,
                y: -self.y,
            },
            Direction::Degrees270 => PositionReal {
                x: self.y,
                y: -self.x,
            },
            Direction::None => panic!("direction should never be none in this application"),
        }
    }
    pub fn integer(self) -> Position {
        Position {
            x: self.x as i64,
            y: self.y as i64,
        }
    }

}

impl core::ops::Add for PositionReal {
    type Output = PositionReal;
    fn add(self, other: PositionReal) -> PositionReal {
        PositionReal {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}