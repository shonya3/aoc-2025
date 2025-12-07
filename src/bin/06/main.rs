use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    println!("Day 6");

    println!(
        "Part 1: {}",
        part1(BufReader::new(File::open("files/06.txt").unwrap())).unwrap()
    );

    let s = std::fs::read_to_string("files/06.txt").unwrap();
    println!("Part 2: {}", part2(&s));
}

#[allow(clippy::needless_range_loop)]
fn part2(input: &str) -> usize {
    let lines_n = input.lines().count();

    let mut max_col: usize = 0;

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let len = chars.len();
            if len > max_col {
                max_col = len;
            };

            chars
        })
        .collect();

    let sign_row_idx = lines_n - 1;

    let mut sign: Option<Sign> = None;
    let mut operands: Vec<usize> = vec![];

    let mut total: usize = 0;

    for col in 0..max_col {
        let s = sign.unwrap_or_else(|| {
            let sign_str = grid[sign_row_idx][col].to_string();
            let s = sign_str.parse().unwrap();
            sign = Some(s);
            s
        });

        if sign.is_none() {
            let sign_str = grid[sign_row_idx][col].to_string();
            sign = Some(sign_str.parse().unwrap());
        }

        let mut value_s = String::new();

        for row in 0..lines_n - 1 {
            let ch = grid[row][col];
            if ch.to_string().parse::<u8>().is_ok() {
                value_s.push(ch);
            }
        }

        // if no values in entire column, operands are ready
        if value_s.is_empty() {
            total += Operation { operands, sign: s }.calc();

            operands = vec![];
            sign = None;
        } else {
            let value: usize = value_s.parse().unwrap();
            operands.push(value);
        }
    }

    if let Some(sign) = sign {
        total += Operation { operands, sign }.calc();
    }

    total
}

#[allow(unused)]
fn part1<R: BufRead>(reader: R) -> Result<usize, Box<dyn Error>> {
    let sum = Input::parse(reader)?
        .into_operations()
        .into_iter()
        .map(|op| op.calc())
        .sum();

    Ok(sum)
}

#[derive(Debug, Clone, Default)]
struct Operation {
    operands: Vec<usize>,
    sign: Sign,
}

impl Operation {
    fn calc(&self) -> usize {
        match self.sign {
            Sign::Add => self.operands.iter().sum(),
            Sign::Mul => {
                let mut product = 1;
                for n in &self.operands {
                    product *= n;
                }

                product
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    nums: Vec<Vec<u16>>,
    signs: Vec<Sign>,
}

impl Input {
    #[allow(clippy::needless_range_loop)]
    fn into_operations(self) -> Vec<Operation> {
        let Input { nums, signs } = self;

        let operands_n = nums.len();
        let problems_n = nums[0].len();

        let mut operations: Vec<Operation> = Vec::new();

        for x in 0..problems_n {
            let mut operands: Vec<usize> = vec![];
            for y in 0..operands_n {
                operands.push(nums[y][x] as usize);
            }

            operations.push(Operation {
                operands,
                sign: signs[x],
            });
        }

        operations
    }

    fn parse<R: BufRead>(reader: R) -> Result<Input, Box<dyn Error>> {
        let mut nums: Vec<Vec<u16>> = Vec::new();

        for line in reader.lines() {
            let line = line?;

            let mut nums_line: Vec<u16> = Vec::new();

            for s in line.split_whitespace() {
                match s.parse::<u16>() {
                    Ok(n) => nums_line.push(n),
                    Err(_) => {
                        let signs = line
                            .split_whitespace()
                            .map(|s| s.parse::<Sign>())
                            .collect::<Result<Vec<Sign>, _>>()?;

                        return Ok(Input { nums, signs });
                    }
                }
            }

            nums.push(nums_line);
        }

        Err("No line of operations signs found".into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum Sign {
    #[default]
    Add,
    Mul,
}

impl FromStr for Sign {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Sign::Add),
            "*" => Ok(Sign::Mul),
            _ => Err(format!("Unexpected sign of operaiton. Expected '+' or '*'. Got {s}").into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;

    const TEST: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +"#;

    #[test]
    fn test_part1() {
        assert_eq!(4277556, part1(TEST.as_bytes()).unwrap())
    }
}
