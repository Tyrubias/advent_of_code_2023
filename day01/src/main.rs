#![forbid(clippy::expect_used)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::panic)]
#![forbid(unsafe_code)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::anyhow;
use once_cell::sync::Lazy;
use utilities::workspace_root;

static NUMBERS: Lazy<HashMap<&str, u32>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    map
});

static LENGTHS: Lazy<HashMap<usize, HashSet<&str>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(3, HashSet::from(["one", "two", "six"]));
    map.insert(4, HashSet::from(["four", "five", "nine"]));
    map.insert(5, HashSet::from(["three", "seven", "eight"]));

    map
});

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(workspace_root()?.join("inputs/day01"))?;

    let sum: u32 = input
        .lines()
        .map(|line| {
            let first_digit = line
                .find(|c: char| c.is_ascii_digit())
                .and_then(|idx| (line.as_bytes()[idx] as char).to_digit(10));
            let last_digit = line
                .rfind(|c: char| c.is_ascii_digit())
                .and_then(|idx| (line.as_bytes()[idx] as char).to_digit(10));

            first_digit
                .zip(last_digit)
                .map(|(first, last)| first * 10 + last)
        })
        .sum::<Option<_>>()
        .ok_or_else(|| anyhow!("invalid sum of calibration values in part one"))?;

    println!("Part one answer: {sum}");

    let total: u32 = input
        .lines()
        .map(|line| {
            let digits = get_digits(line);

            digits.map(|(first_digit, last_digit)| first_digit * 10 + last_digit)
        })
        .sum::<Option<_>>()
        .ok_or_else(|| anyhow!("invalid sum of calibration values in part two"))?;

    println!("Part two answer: {total}");

    Ok(())
}

fn get_digits(line: &str) -> Option<(u32, u32)> {
    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut first_found = false;
    let mut last_found = false;

    'first: for (idx, num) in line.char_indices() {
        if num.is_ascii_digit() {
            first_digit = num.to_digit(10)?;
            first_found = true;
            break;
        }
        for (&length, set) in LENGTHS.iter() {
            if let Some(num) = line.get(idx..idx + length) {
                if set.contains(num) {
                    first_digit = *NUMBERS.get(num)?;
                    first_found = true;
                    break 'first;
                }
            }
        }
    }

    'last: for (idx, num) in line.char_indices().rev() {
        if num.is_ascii_digit() {
            last_digit = num.to_digit(10)?;
            last_found = true;
            break;
        }
        for (&length, set) in LENGTHS.iter() {
            let end = idx + 1;
            if let Some(start) = end.checked_sub(length) {
                if let Some(num) = line.get(start..end) {
                    if set.contains(num) {
                        last_digit = *NUMBERS.get(num)?;
                        last_found = true;
                        break 'last;
                    }
                }
            }
        }
    }

    if first_found && last_found {
        Some((first_digit, last_digit))
    } else {
        None
    }
}
