use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

const MAX: u16 = 99;

fn main() {
    println!(
        "Part1: {}",
        part1(BufReader::new(File::open("files/01.txt").unwrap())).unwrap()
    );

    println!(
        "Part2: {}",
        part2(BufReader::new(File::open("files/01.txt").unwrap())).unwrap()
    )
}

fn part1<R: BufRead>(reader: R) -> Result<usize, Box<dyn Error>> {
    let mut dial = Dial::default();

    Ok(read_input(reader)?
        .into_iter()
        .filter(|rotation| {
            dial.rotate(*rotation);
            dial.point == 0
        })
        .count())
}

fn part2<R: BufRead>(reader: R) -> Result<usize, Box<dyn Error>> {
    let mut dial = Dial::default();

    Ok(read_input(reader)?
        .into_iter()
        .map(|rotation| dial.rotate_part2(rotation))
        .sum())
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    L,
    R,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotation {
    pub direction: Direction,
    pub value: u16,
}

impl FromStr for Rotation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d_str, v_str) = s.split_at(1);

        let direction = match d_str {
            "L" => Direction::L,
            "R" => Direction::R,
            _ => return Err(format!("Invalid direction: {d_str}").into()),
        };

        let value = v_str.parse()?;

        Ok(Rotation { direction, value })
    }
}

pub struct Dial {
    pub point: u16,
}

impl Dial {
    pub const fn new(point: u16) -> Dial {
        Dial { point }
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let value = rotation.value % (MAX + 1);

        match rotation.direction {
            Direction::L => {
                if self.point >= value {
                    self.point -= value;
                } else {
                    let remainder = value - self.point;
                    self.point = MAX - remainder + 1;
                }
            }
            Direction::R => {
                let sum = self.point + value;

                if sum <= MAX {
                    self.point = sum;
                } else {
                    self.point = sum - MAX - 1;
                }
            }
        }
    }

    /// Returns a number of times dial pointed at 0, regardless of whether it happens
    /// during a rotation or at the end of one.
    pub fn rotate_part2(&mut self, rotation: Rotation) -> usize {
        let value = rotation.value % (MAX + 1);

        let full_spins = (rotation.value / (MAX + 1)) as usize;

        let pointed_at_zero = match rotation.direction {
            Direction::L => {
                if self.point >= value {
                    self.point -= value;
                    self.point == 0
                } else {
                    let was_zero = self.point == 0;
                    let remainder = value - self.point;
                    self.point = MAX - remainder + 1;
                    !was_zero
                }
            }
            Direction::R => {
                let sum = self.point + value;

                if sum <= MAX {
                    self.point = sum;
                    self.point == 0
                } else {
                    let was_zero = self.point == 0;
                    self.point = sum - MAX - 1;
                    !was_zero
                }
            }
        };

        full_spins + (if pointed_at_zero { 1 } else { 0 })
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self::new(50)
    }
}

#[allow(unused)]
fn read_input<R: BufRead>(reader: R) -> Result<Vec<Rotation>, Box<dyn Error>> {
    reader.lines().map(|line| line?.parse()).collect()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::{Dial, Direction, Rotation, part1, part2, read_input};

    const TEST: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_read_input() {
        let rotation = read_input(BufReader::new(TEST.as_bytes())).unwrap();

        assert_eq!(
            Some(Rotation {
                direction: Direction::L,
                value: 68,
            }),
            rotation.first().copied()
        );

        assert_eq!(
            Some(Rotation {
                direction: Direction::L,
                value: 82,
            }),
            rotation.last().copied()
        );
    }

    #[test]
    fn test_rotation() {
        let mut dial = Dial::default();
        let rotations = read_input(TEST.as_bytes()).unwrap();

        assert_eq!(dial.point, 50);

        dial.rotate(rotations[0]); // L68
        assert_eq!(82, dial.point);

        dial.rotate(rotations[1]); // L30
        assert_eq!(52, dial.point);

        dial.rotate(rotations[2]); // R48
        assert_eq!(0, dial.point);

        dial.rotate(rotations[3]); // L5
        assert_eq!(95, dial.point);

        dial.rotate(rotations[4]); // R60
        assert_eq!(55, dial.point);

        dial.rotate(rotations[5]); // L55
        assert_eq!(0, dial.point);

        dial.rotate(rotations[6]); // L1
        assert_eq!(99, dial.point);

        dial.rotate(rotations[7]); // L99
        assert_eq!(0, dial.point);

        dial.rotate(rotations[8]); // R14
        assert_eq!(14, dial.point);

        dial.rotate(rotations[9]); // L82
        assert_eq!(32, dial.point);
    }

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(BufReader::new(TEST.as_bytes())).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(TEST.as_bytes()).unwrap());
    }
}
