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
}

#[allow(clippy::needless_range_loop)]
fn part1<R: BufRead>(reader: R) -> Result<usize, Box<dyn Error>> {
    let Input { nums, ops } = parse(reader)?;

    let operands_n = nums.len();
    let problems_n = nums[0].len();

    let mut sum = 0;

    for x in 0..problems_n {
        let mut vec: Vec<usize> = vec![];
        for y in 0..operands_n {
            vec.push(nums[y][x] as usize);
        }

        sum += match ops[x] {
            Op::Add => vec.iter().sum(),
            Op::Mul => {
                let mut product = 1;
                for n in vec {
                    product *= n;
                }

                product
            }
        };
    }

    Ok(sum)
}

#[derive(Debug, Clone)]
struct Input {
    nums: Vec<Vec<u16>>,
    ops: Vec<Op>,
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
                    let ops = line
                        .split_whitespace()
                        .map(|s| s.parse::<Op>())
                        .collect::<Result<Vec<Op>, _>>()?;

                    return Ok(Input { nums, ops });
                }
            }
        }

        nums.push(nums_line);
    }

    Err("No line of operations symbols found".into())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => {
                Err(format!("Unexpected symbol of operaiton. Expected '+' or '*'. Got {s}").into())
            }
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
