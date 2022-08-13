use std::str::FromStr;

pub enum Course {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Default)]
pub struct SubMarine {
    aim: usize,
    depth: usize,
    hpos: usize,
}

impl SubMarine {
    pub fn advance(&mut self, course: &Course) {
        match course {
            Course::Forward(len) => {
                self.hpos += len;
                self.depth += self.aim * len;
            }
            Course::Down(len) => self.aim += len,
            Course::Up(len) => self.aim -= len,
        }
    }
    pub fn advance_without_aim(&mut self, course: &Course) {
        match course {
            Course::Forward(len) => self.hpos += len,
            Course::Down(len) => self.depth += len,
            Course::Up(len) => self.depth -= len,
        }
    }
    pub fn final_destination(&self) -> usize {
        self.hpos * self.depth
    }
}

#[derive(Debug)]
pub enum VectorParseError {
    InvalidDirection(String),
    InvalidLength(String),
    Invalidline(String),
}

impl FromStr for Course {
    type Err = VectorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((dir, len_str)) = s.trim().split_once(' ') {
            if let Ok(len) = len_str.parse::<usize>() {
                match dir {
                    "forward" => Ok(Course::Forward(len)),
                    "down" => Ok(Course::Down(len)),
                    "up" => Ok(Course::Up(len)),
                    _ => Err(VectorParseError::InvalidDirection(dir.to_string())),
                }
            } else {
                Err(VectorParseError::Invalidline(len_str.to_string()))
            }
        } else {
            Err(VectorParseError::Invalidline(s.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn move_without_aim() {
        let moves = [
            Course::Forward(5),
            Course::Down(5),
            Course::Forward(8),
            Course::Up(3),
            Course::Down(8),
            Course::Forward(2),
        ];
        let mut sm = SubMarine::default();
        moves
            .iter()
            .for_each(|movement| sm.advance_without_aim(movement));
        assert_eq!(sm.final_destination(), 150);
    }
    #[test]
    fn move_with_aim() {
        let moves = [
            Course::Forward(5),
            Course::Down(5),
            Course::Forward(8),
            Course::Up(3),
            Course::Down(8),
            Course::Forward(2),
        ];
        let mut sm = SubMarine::default();
        moves.iter().for_each(|movement| sm.advance(movement));
        assert_eq!(sm.final_destination(), 900);
    }
}
