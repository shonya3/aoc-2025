use std::{
    error::Error,
    fmt::{Display, Write},
    str::FromStr,
};

fn main() {
    // println!("Day 7");

    let s = std::fs::read_to_string("files/07.txt").unwrap();

    println!("Part 1: {}", part1(&s).unwrap());
}

fn part1(s: &str) -> Result<usize, Box<dyn Error>> {
    let mut grid: Grid = s.parse()?;

    let start_pos = Position {
        row: 0,
        col: grid.grid[0]
            .iter()
            .position(|&f| f == Field::Start)
            .unwrap(),
    };

    let mut beams = vec![Beam { pos: start_pos }];

    let mut splits_n = 0;

    while let Some(mut beam) = beams.pop() {
        let result = beam.walk(&mut grid);
        match result.kind {
            BeamResultKind::Split => {
                splits_n += 1;
                if beam.pos.col > 0 {
                    let left_pos = Position {
                        row: beam.pos.row,
                        col: beam.pos.col - 1,
                    };

                    if let Some(left_field) = grid.get(left_pos)
                        && matches!(left_field, Field::Empty)
                    {
                        beams.push(Beam { pos: left_pos });
                    }
                }

                let right_pos = Position {
                    row: beam.pos.row,
                    col: beam.pos.col + 1,
                };

                if let Some(right_field) = grid.get(right_pos)
                    && matches!(right_field, Field::Empty)
                {
                    beams.push(Beam { pos: right_pos });
                }
            }
            BeamResultKind::EndOfGrid => {}
            BeamResultKind::BeamOverlap => {}
        }
    }

    Ok(splits_n)
}

#[derive(Debug)]
struct Beam {
    pos: Position,
}

impl Beam {
    fn walk(&mut self, grid: &mut Grid) -> BeamResult {
        loop {
            let next_pos = Position {
                row: self.pos.row + 1,
                col: self.pos.col,
            };

            *grid.get_mut(self.pos).unwrap() = Field::Beam;

            let Some(field) = grid.get_mut(next_pos) else {
                return BeamResult {
                    position: self.pos,
                    kind: BeamResultKind::EndOfGrid,
                };
            };

            match field {
                Field::Empty => {
                    self.pos = next_pos;
                }
                Field::Start => panic!("Unexpected step field: Start"),
                Field::Splitter => {
                    return BeamResult {
                        position: self.pos,
                        kind: BeamResultKind::Split,
                    };
                }
                Field::Beam => {
                    // println!("Position: {next_pos:?}");
                    // println!("{grid}");
                    // panic!("Unexpected step field: Beam")
                    return BeamResult {
                        position: self.pos,
                        kind: BeamResultKind::BeamOverlap,
                    };
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct BeamResult {
    #[allow(unused)]
    position: Position,
    kind: BeamResultKind,
}

#[derive(Debug, Clone, Copy)]
enum BeamResultKind {
    Split,
    EndOfGrid,
    BeamOverlap,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<Field>>,
}

impl Grid {
    fn get(&self, pos: Position) -> Option<Field> {
        self.grid
            .get(pos.row)
            .and_then(|row| row.get(pos.col))
            .copied()
    }

    fn get_mut(&mut self, pos: Position) -> Option<&mut Field> {
        self.grid
            .get_mut(pos.row)
            .and_then(|row| row.get_mut(pos.col))
    }
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| -> Result<Vec<Field>, Self::Err> {
                line.chars()
                    .map(|ch| ch.to_string().parse::<Field>())
                    .collect()
            })
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(Grid { grid })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_row_idx = self.grid.len() - 1;
        for (cur_row_idx, row) in self.grid.iter().enumerate() {
            for field in row.iter() {
                field.fmt(f)?;
            }

            if cur_row_idx != max_row_idx {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Field {
    Empty,
    Start,
    Splitter,
    Beam,
}

impl Field {
    pub fn as_char(&self) -> char {
        match self {
            Field::Empty => '.',
            Field::Start => 'S',
            Field::Splitter => '^',
            Field::Beam => '|',
        }
    }
}

impl FromStr for Field {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Field::Empty),
            "S" => Ok(Field::Start),
            "^" => Ok(Field::Splitter),
            "|" => Ok(Field::Beam),
            _ => Err(format!("Unexpected field symbol: {s}").into()),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

#[cfg(test)]
mod tests {
    use crate::{Grid, part1};

    const TEST: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_parse() {
        assert_eq!(TEST, TEST.parse::<Grid>().unwrap().to_string().as_str())
    }

    #[test]
    fn test_part1() {
        assert_eq!(21, part1(TEST).unwrap())
    }
}
