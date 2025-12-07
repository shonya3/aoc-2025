use crate::part2::part2;
use std::{error::Error, ops::RangeInclusive, str::FromStr};

fn main() {
    println!("Hello, World!");

    let s = std::fs::read_to_string("files/05.txt").unwrap();

    println!("Part 1: {}", part1(&s).unwrap());
    println!("Part 2: {}", part2(&s).unwrap());
}

#[allow(unused)]
fn part1(s: &str) -> Result<usize, Box<dyn Error>> {
    let database: Database = s.parse()?;

    Ok(database
        .available
        .iter()
        .filter(|&&f| database.is_fresh(f))
        .count())
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Database {
    fresh: Vec<RangeInclusive<usize>>,
    available: Vec<usize>,
}

impl Database {
    fn is_fresh(&self, a: usize) -> bool {
        for range in &self.fresh {
            if is_in_range(a, range) {
                return true;
            }
        }

        false
    }
}

impl FromStr for Database {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fresh = Vec::new();
        let mut available = Vec::new();
        let mut parsing_fresh = true;

        for line in s.lines() {
            if line.is_empty() {
                parsing_fresh = false;
                continue;
            }

            if parsing_fresh {
                fresh.push(parse_range(line)?);
            } else {
                available.push(line.parse()?);
            }
        }

        Ok(Database { fresh, available })
    }
}

fn parse_range(input: &str) -> Result<RangeInclusive<usize>, Box<dyn Error>> {
    let (start_str, end_str) = input.split_once('-').ok_or(format!(
        "Invalid input(): no dash-symbol(-). Input: {input}"
    ))?;

    let start = start_str.parse()?;
    let end = end_str.parse()?;

    Ok(start..=end)
}

fn is_in_range(a: usize, range: &RangeInclusive<usize>) -> bool {
    a >= *range.start() && a <= *range.end()
}

#[cfg(test)]
mod tests {
    use crate::{Database, is_in_range, part1};

    const TEST: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    fn test_db() -> Database {
        TEST.parse().unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(TEST).unwrap())
    }

    #[test]
    fn test_parse_database() {
        assert_eq!(
            Database {
                fresh: vec![3..=5, 10..=14, 16..=20, 12..=18],
                available: vec![1, 5, 8, 11, 17, 32],
            },
            test_db()
        );
    }

    #[test]
    fn test_is_in_range() {
        assert!(!is_in_range(1, &(3..=5)));
        assert!(is_in_range(3, &(3..=5)));
        assert!(is_in_range(5, &(3..=5)));
    }

    #[test]
    fn test_is_fresh() {
        let db = test_db();

        assert!(!db.is_fresh(1));
        assert!(db.is_fresh(5));
        assert!(!db.is_fresh(8));
        assert!(db.is_fresh(11));
        assert!(db.is_fresh(17));
        assert!(!db.is_fresh(32));
    }
}

mod part2 {
    use crate::Database;
    use std::{cmp, error::Error};

    pub fn part2(s: &str) -> Result<usize, Box<dyn Error>> {
        let database: Database = s.parse()?;

        let ranges: Vec<RangeInc> = database
            .fresh
            .into_iter()
            .map(|range| RangeInc {
                start: *range.start(),
                end: *range.end(),
            })
            .collect();

        Ok(merge_overlapping_ranges(ranges)
            .iter()
            .map(|r| r.count())
            .sum())
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RangeInc {
        start: usize,
        end: usize,
    }

    impl RangeInc {
        fn count(&self) -> usize {
            self.end - self.start + 1
        }
    }

    fn merge_overlapping_ranges(mut ranges: Vec<RangeInc>) -> Vec<RangeInc> {
        if ranges.is_empty() {
            return vec![];
        }

        ranges.sort_by_key(|r| r.start);

        let mut iter = ranges.into_iter();
        let mut merged = vec![iter.next().unwrap()];

        for next_range in iter {
            if let Some(last_merged) = merged.last_mut() {
                if next_range.start <= last_merged.end {
                    last_merged.end = cmp::max(last_merged.end, next_range.end);
                } else {
                    merged.push(next_range);
                }
            }
        }

        merged
    }
}
