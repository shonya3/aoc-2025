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

    println!(
        "Part 2: {}",
        part2(BufReader::new(File::open("files/03.txt").unwrap())).unwrap()
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

fn part2<R: BufRead>(reader: R) -> Result<u64, Box<dyn Error>> {
    parse_input(reader)?
        .into_iter()
        .map(|bank| {
            let values = bank.find_n_highest(12);
            let s: String = values.into_iter().map(|v| v.to_string()).collect();
            Ok(s.parse::<u64>()?)
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

    fn find_n_highest(&self, n: usize) -> Vec<u8> {
        let mut start = 0;
        let mut values = vec![];

        for r in (1..=n).rev() {
            let idx = self.find_highest_index(start, r);
            let value = self.batteries[idx];
            values.push(value);
            start = idx + 1;
        }

        values
    }

    fn find_highest_index(&self, start: usize, remaining_n_to_find: usize) -> usize {
        let len = self.batteries.len();

        let mut max = 1;
        let mut max_idx = start;

        for (i, a) in self.batteries[start..=(len - remaining_n_to_find)]
            .iter()
            .enumerate()
        {
            if *a > max {
                max = *a;
                max_idx = start + i;
            }
        }

        max_idx
    }

    fn find_highest_pair_indices(&self) -> (usize, usize) {
        let len: usize = self.batteries.len();

        for a_candidate in (1..=9).rev() {
            for (i, batterry_a) in self.batteries.iter().take(len - 1).enumerate() {
                if *batterry_a == a_candidate {
                    for b_candidate in (1..=9).rev() {
                        for (j, batterry_b) in self.batteries.iter().skip(i + 1).enumerate() {
                            if *batterry_b == b_candidate {
                                return (i, i + 1 + j);
                            }
                        }
                    }
                }
            }
        }

        (len - 2, len - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bank, parse_input, part1, part2};

    const TEST: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part1() {
        assert_eq!(357, part1(TEST.as_bytes()).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3121910778619, part2(TEST.as_bytes()).unwrap());
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

    #[test]
    fn test_find_highest_index() {
        let bank = |s: &str| -> Bank { s.parse().unwrap() };

        assert_eq!(0, bank("811111111111119").find_highest_index(0, 12));
        assert_eq!(14, bank("234234234234278").find_highest_index(14, 1));
        assert_eq!(14, bank("811111111111119").find_highest_index(12, 1));
        assert_eq!(5, bank("987654321111111").find_highest_index(5, 7));
    }

    #[test]
    fn test_find_n_highest() {
        let bank = |s: &str| -> Bank { s.parse().unwrap() };

        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1],
            bank("987654321111111").find_n_highest(12)
        );

        assert_eq!(
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            bank("811111111111119").find_n_highest(12)
        );
        assert_eq!(
            vec![4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            bank("234234234234278").find_n_highest(12)
        );
        assert_eq!(
            vec![8, 8, 8, 9, 1, 1, 1, 1, 2, 1, 1, 1],
            bank("818181911112111").find_n_highest(12)
        );
    }
}
