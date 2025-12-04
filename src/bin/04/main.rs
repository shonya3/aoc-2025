use std::{error::Error, fmt::Display, str::FromStr};

fn main() {
    println!("Day 4");

    let s = std::fs::read_to_string("files/04.txt").unwrap();

    println!("Part 1: {}", part1(&s));
    println!("Part 2: {}", part2(&s));
}

fn part1(s: &str) -> usize {
    s.parse::<Grid>().unwrap().get_removable_positions().len()
}

fn part2(s: &str) -> usize {
    let mut grid: Grid = s.parse().unwrap();

    let mut total_removed = 0;

    loop {
        let positions = grid.get_removable_positions();

        if positions.is_empty() {
            return total_removed;
        }

        total_removed += positions.len();

        for pos in positions {
            grid.grid[pos.y][pos.x] = Field::Empty;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    grid: Vec<Vec<Field>>,
}

impl Grid {
    fn get(&self, pos: Position) -> Option<Field> {
        self.grid.get(pos.y).and_then(|row| row.get(pos.x)).copied()
    }

    /// Removable: number of adjacent paper fields < 4
    fn get_removable_positions(&self) -> Vec<Position> {
        self.clone()
            .grid
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| row.into_iter().enumerate().map(move |(x, f)| (f, p(x, y))))
            .filter(|(f, _)| f == &Field::Paper)
            .filter_map(|(_, pos)| match self.get_adjacent_paper_n(pos.clone()) {
                0..=3 => Some(pos),
                _ => None,
            })
            .collect()
    }

    fn get_adjacent_paper_n(&self, pos: Position) -> usize {
        let can_top = pos.y > 0;
        let can_left = pos.x > 0;

        let mut positions: Vec<Position> = vec![];

        // top
        if can_top {
            let y = pos.y - 1;
            if can_left {
                positions.push(p(pos.x - 1, y));
            }
            positions.push(p(pos.x, y));
            positions.push(p(pos.x + 1, y));
        }

        // mid
        let y = pos.y;
        if can_left {
            positions.push(p(pos.x - 1, y));
        }
        positions.push(p(pos.x + 1, y));

        // bot
        let y = pos.y + 1;
        if can_left {
            positions.push(p(pos.x - 1, y));
        }
        positions.push(p(pos.x, y));
        positions.push(p(pos.x + 1, y));

        positions
            .into_iter()
            .filter(|pos| self.get(pos.clone()) == Some(Field::Paper))
            .count()
    }
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| -> Result<Vec<Field>, Box<dyn Error>> {
                line.chars()
                    .map(|ch| ch.to_string().parse::<Field>())
                    .collect()
            })
            .collect::<Result<Vec<Vec<Field>>, Box<dyn Error>>>()?;

        Ok(Grid { grid })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn p(x: usize, y: usize) -> Position {
    Position { x, y }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Field {
    Empty,
    Paper,
}

impl FromStr for Field {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Field::Empty),
            "@" => Ok(Field::Paper),
            _ => Err(format!("Invalid field symbol. Expected '@' or '.', but got {s}").into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Field, Grid, p, part1, part2};

    const TEST: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    fn test_grid() -> Grid {
        TEST.parse().unwrap()
    }

    const E: Field = Field::Empty;
    const P: Field = Field::Paper;

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(TEST))
    }

    #[test]
    fn test_part2() {
        assert_eq!(43, part2(TEST))
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(vec![E, E, P, P, E, P, P, P, P, E], test_grid().grid[0]);
    }

    #[test]
    fn test_grid_get() {
        let grid = test_grid();
        assert_eq!(Some(E), grid.get(p(4, 0)));
        assert_eq!(None, grid.get(p(10, 0)));
    }
}
