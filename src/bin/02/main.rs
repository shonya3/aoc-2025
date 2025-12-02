use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn main() {
    println!("Day 2");

    println!(
        "Part 1: {}",
        part1(BufReader::new(File::open("files/02.txt").unwrap())).unwrap()
    );

    println!(
        "Part 2: {}",
        part2(BufReader::new(File::open("files/02.txt").unwrap())).unwrap()
    );
}

fn part1<R: BufRead>(reader: R) -> Result<usize, Box<dyn Error>> {
    Ok(parse_input(reader)?
        .into_iter()
        .flat_map(find_invalid_ids)
        .sum())
}

fn part2<R: BufRead>(reader: R) -> Result<usize, Box<dyn Error>> {
    Ok(parse_input(reader)?
        .into_iter()
        .flat_map(find_invalid_ids_part2)
        .sum())
}

fn parse_input<R: BufRead>(mut reader: R) -> Result<Vec<RangeInclusive<usize>>, Box<dyn Error>> {
    let mut line = String::new();
    reader.read_line(&mut line)?;

    line.trim().split(',').map(parse_range).collect()
}

fn parse_range(input: &str) -> Result<RangeInclusive<usize>, Box<dyn Error>> {
    let (start_str, end_str) = input.split_once('-').ok_or(format!(
        "Invalid input(): no dash-symbol(-). Input: {input}"
    ))?;

    let start = start_str.parse()?;
    let end = end_str.parse()?;

    Ok(start..=end)
}

fn find_invalid_ids(ids: RangeInclusive<usize>) -> Vec<usize> {
    ids.into_iter()
        .filter(|&num| is_repeated_twice(num))
        .collect()
}

fn find_invalid_ids_part2(ids: RangeInclusive<usize>) -> Vec<usize> {
    ids.into_iter().filter(|&num| is_repeated(num)).collect()
}

fn is_repeated_twice(num: usize) -> bool {
    let s = num.to_string();

    let len = s.len();
    if !len.is_multiple_of(2) {
        return false;
    }

    let (l, r) = s.split_at(len / 2);

    l == r
}

const MAX_CHUNK_SIZE: usize = 30;
fn is_repeated(num: usize) -> bool {
    let s = num.to_string();

    // A pattern must appear at least twice, so we only need to check for
    // chunk lengths up to half the total string length.
    for chunk_size in 1..=s.len() / 2 {
        if chunk_size > MAX_CHUNK_SIZE {
            break;
        }

        if is_repeated_chunk_size(&s, chunk_size) {
            return true;
        }
    }

    false
}

fn is_repeated_chunk_size(s: &str, chunk_size: usize) -> bool {
    if chunk_size == 0 {
        return s.is_empty();
    }

    if !s.len().is_multiple_of(chunk_size) {
        return false;
    }

    let mut chunks = s.as_bytes().chunks(chunk_size);

    chunks
        .next()
        .is_none_or(|first| chunks.all(|chunk| chunk == first))
}

#[cfg(test)]
mod tests {
    use crate::{
        find_invalid_ids, is_repeated, is_repeated_chunk_size, is_repeated_twice, parse_input,
        parse_range, part1, part2,
    };

    const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(1227775554, part1(TEST.as_bytes()).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(4174379265, part2(TEST.as_bytes()).unwrap());
    }

    #[test]
    fn test_find_invalid_ids() {
        assert_eq!(vec![11, 22], find_invalid_ids(11..=22));
        assert_eq!(vec![99], find_invalid_ids(99..=115));
        assert_eq!(vec![1010], find_invalid_ids(998..=1012));
        assert_eq!(vec![1188511885], find_invalid_ids(1188511880..=1188511890));
        assert_eq!(vec![222222], find_invalid_ids(222220..=222224));
        assert_eq!(Vec::<usize>::new(), find_invalid_ids(1698522..=1698528));
        assert_eq!(vec![446446], find_invalid_ids(446443..=446449));
        assert_eq!(vec![38593859], find_invalid_ids(38593856..=38593862));
        assert_eq!(Vec::<usize>::new(), find_invalid_ids(565653..=565659));
        assert_eq!(Vec::<usize>::new(), find_invalid_ids(824824821..=824824827));
        assert_eq!(
            Vec::<usize>::new(),
            find_invalid_ids(2121212118..=2121212124)
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            vec![11..=22, 5142771457..=5142940464],
            parse_input("11-22,5142771457-5142940464".as_bytes()).unwrap()
        )
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(11..=22, parse_range("11-22").unwrap());
        assert_eq!(
            5142771457..=5142940464,
            parse_range("5142771457-5142940464").unwrap()
        );
    }

    #[test]
    fn test_is_repeated_twice() {
        assert!(is_repeated_twice(22));
        assert!(is_repeated_twice(123123));
        assert!(!is_repeated_twice(1235123));
    }

    #[test]
    fn test_is_repeated_chunk_size() {
        assert!(is_repeated_chunk_size("12121212", 2));
        assert!(is_repeated_chunk_size("12341234", 4));
        assert!(!is_repeated_chunk_size("12341234", 2));
    }

    #[test]
    fn test_is_repeated() {
        assert!(is_repeated(12121212));
        assert!(is_repeated(12341234));
        assert!(!is_repeated(123451234));
        assert!(!is_repeated(12));
    }
}
