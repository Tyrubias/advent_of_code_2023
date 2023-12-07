#![forbid(clippy::expect_used)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::panic)]
#![forbid(unsafe_code)]

use std::{collections::HashSet, fs};

use regex::Regex;
use utilities::workspace_root;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(workspace_root()?.join("inputs/day03"))?;

    let symbol_coords: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter(|(_, c)| !(c.is_ascii_digit() || *c == '.'))
                .map(move |(j, _)| (i as i64, j as i64))
        })
        .collect();

    let digit_re = Regex::new(r"\d+")?;

    let part_numbers = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            let symbol_coords = &symbol_coords;
            digit_re
                .find_iter(line)
                .filter(move |r#match| {
                    let i = i as i64;
                    let start = r#match.start() as i64;
                    let end = r#match.end() as i64;

                    let mut is_part_number = (start - 1..=end)
                        .flat_map(|j| [(i - 1, j), (i + 1, j)].into_iter())
                        .any(|pair| symbol_coords.contains(&pair));

                    is_part_number = is_part_number
                        || (symbol_coords.contains(&(i, start - 1))
                            || symbol_coords.contains(&(i, end)));

                    is_part_number
                })
                .map(|r#match| r#match.as_str().parse::<u32>())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let part_sum: u32 = part_numbers.iter().sum();

    println!("Part one answer: {part_sum}");

    Ok(())
}
