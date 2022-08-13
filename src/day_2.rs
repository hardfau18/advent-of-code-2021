use std::str::FromStr;

pub enum Direction {
    Forward,
    Down,
    Up,
}
pub struct Vector {
    direction: Direction,
    len: usize,
}

#[derive(Debug)]
pub enum VectorParseError {
    InvalidWord(String),
    NoDirection,
    NoLength,
}

impl FromStr for Vector {
    type Err = VectorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.trim().split(' ');
        let direction = if let Some(word) = line.next() {
            match word {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => return Err(VectorParseError::InvalidWord(word.to_string())),
            }
        } else {
            return Err(VectorParseError::NoDirection);
        };
        let len = if let Some(word) = line.next() {
            word.parse()
                .map_err(|_| VectorParseError::InvalidWord(word.to_string()))?
        } else {
            return Err(VectorParseError::NoLength);
        };
        Ok(Self { direction, len })
    }
}

impl Vector {
    pub fn new(dir: Direction, len: usize) -> Self {
        Self {
            direction: dir,
            len,
        }
    }
}
pub fn final_destination(moves: &[Vector]) -> usize {
    let (x, y) = moves
        .iter()
        .fold::<(usize, usize), _>((0, 0), |acc, mov| match mov.direction {
            Direction::Forward => (acc.0 + mov.len, acc.1),
            Direction::Down => (acc.0, acc.1 + mov.len),
            Direction::Up => (acc.0, acc.1 - mov.len),
        });
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day2_example() {
        let moves = [
            Vector::new(Direction::Forward, 5),
            Vector::new(Direction::Down, 5),
            Vector::new(Direction::Forward, 8),
            Vector::new(Direction::Up, 3),
            Vector::new(Direction::Down, 8),
            Vector::new(Direction::Forward, 2),
        ];
        assert_eq!(final_destination(&moves), 150);
    }
}
