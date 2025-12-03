use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    println!("Day 3");

    println!(
        "Part 1: {}",
        part1(BufReader::new(File::open("files/03.txt").unwrap())).unwrap()
    );
}

fn part1<R: BufRead>(reader: R) -> Result<u64, Box<dyn Error>> {
    parse_input(reader)?
        .into_iter()
        .map(|bank| {
            let (a, b) = bank.find_highest_pair();
            Ok(format!("{a}{b}").parse::<u64>()?)
        })
        .sum()
}

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<Bank>, Box<dyn Error>> {
    reader.lines().map(|line| line?.parse()).collect()
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Bank {
    batteries: Vec<u8>,
}

impl FromStr for Bank {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        Ok(Bank { batteries })
    }
}

impl Bank {
    #[allow(unused)]
    const fn new(batteries: Vec<u8>) -> Bank {
        Bank { batteries }
    }

    fn find_highest_pair(&self) -> (u8, u8) {
        self.get_pair(self.find_highest_pair_indices())
    }

    fn get_pair(&self, (a_idx, b_idx): (usize, usize)) -> (u8, u8) {
        (self.batteries[a_idx], self.batteries[b_idx])
    }

    fn find_highest_pair_indices(&self) -> (usize, usize) {
        let len: usize = self.batteries.len();

        for a_candidate in (1..=9).rev() {
            for (i, batterry_a) in self.batteries.iter().take(len - 1).enumerate() {
                // println!("i: {i} batterry_a: {batterry_a} a_candidate:{a_candidate}");
                if *batterry_a == a_candidate {
                    // println!("A FOUND: i: {i} batterry_a: {batterry_a} a_candidate:{a_candidate}");

                    for b_candidate in (1..=9).rev() {
                        for (j, batterry_b) in self.batteries.iter().skip(i + 1).enumerate() {
                            if *batterry_b == b_candidate {
                                // println!(
                                //     "B FOUND: i+1+j: {} batterry_b: {batterry_b} b_candidate:{b_candidate}",
                                //     { i + 1 + j }
                                // );

                                return (i, i + 1 + j);
                            }
                        }
                    }

                    // return (1, 1);
                }
            }
        }

        (len - 2, len - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bank, parse_input, part1};

    const TEST: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part1() {
        assert_eq!(357, part1(TEST.as_bytes()).unwrap());
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            parse_input(TEST.as_bytes()).unwrap()[0].batteries
        );
    }

    #[test]
    fn test_find_highest_pair_indices() {
        let bank = |s: &str| -> Bank { s.parse().unwrap() };

        assert_eq!((0, 1), bank("987654321111111").find_highest_pair_indices());
        assert_eq!((0, 14), bank("811111111111119").find_highest_pair_indices());
        assert_eq!(
            (13, 14),
            bank("234234234234278").find_highest_pair_indices()
        );
        assert_eq!((6, 11), bank("818181911112111").find_highest_pair_indices());
    }
}
