#![forbid(clippy::expect_used)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::panic)]
#![forbid(unsafe_code)]

use std::{collections::HashMap, fs};

use anyhow::anyhow;
use regex::Regex;
use utilities::workspace_root;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(workspace_root()?.join("inputs/day03"))?;

    let mut symbol_coords: HashMap<_, Vec<u32>> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter(|(_, c)| !(c.is_ascii_digit() || *c == '.'))
                .map(move |(j, _)| ((i as i64, j as i64), Vec::new()))
        })
        .collect();

    let re = Regex::new(r"\d+")?;

    for (i, line) in input.lines().enumerate() {
        let i = i as i64;
        for r#match in re.find_iter(line) {
            let start = r#match.start() as i64;
            let end = r#match.end() as i64;
            let part_number: u32 = r#match.as_str().parse()?;

            for j in start - 1..=end {
                if symbol_coords.contains_key(&(i - 1, j)) {
                    symbol_coords
                        .get_mut(&(i - 1, j))
                        .ok_or_else(|| anyhow!("symbol table must contain coordinates"))?
                        .push(part_number);
                }
                if symbol_coords.contains_key(&(i + 1, j)) {
                    symbol_coords
                        .get_mut(&(i + 1, j))
                        .ok_or_else(|| anyhow!("symbol table must contain coordinates"))?
                        .push(part_number);
                }
            }
            if symbol_coords.contains_key(&(i, start - 1)) {
                symbol_coords
                    .get_mut(&(i, start - 1))
                    .ok_or_else(|| anyhow!("symbol table must contain coordinates"))?
                    .push(part_number);
            }
            if symbol_coords.contains_key(&(i, end)) {
                symbol_coords
                    .get_mut(&(i, end))
                    .ok_or_else(|| anyhow!("symbol table must contain coordinates"))?
                    .push(part_number);
            }
        }
    }

    let part_number_sum: u32 = symbol_coords
        .values()
        .map(|list| list.iter().sum::<u32>())
        .sum();

    println!("Part one answer: {part_number_sum}");

    let gear_ratio_sum: u32 = symbol_coords
        .values()
        .filter(|list| list.len() == 2)
        .map(|list| list.iter().product::<u32>())
        .sum();

    println!("Part two answer: {gear_ratio_sum}");

    Ok(())
}
