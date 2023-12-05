#![forbid(clippy::expect_used)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::panic)]
#![forbid(unsafe_code)]

use std::fs;

use anyhow::anyhow;
use itertools::multiunzip;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};
use utilities::workspace_root;

const MAX_SUBSET: Subset = Subset {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(workspace_root()?.join("inputs/day02"))?;

    let games = input
        .lines()
        .map(|line| {
            all_consuming(parse_game)(line)
                .finish()
                .map(|(_, game)| game)
                .map_err(|error| nom::error::Error::new(error.input.to_string(), error.code))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let id_total: u32 = games
        .iter()
        .filter(|game| {
            game.subsets
                .iter()
                .filter(|subset| {
                    subset.red > MAX_SUBSET.red
                        || subset.green > MAX_SUBSET.green
                        || subset.blue > MAX_SUBSET.blue
                })
                .count()
                == 0
        })
        .map(|game| game.id)
        .sum();

    println!("Part one answer: {id_total}");

    let power_total: Option<u32> = games
        .iter()
        .map(|game| {
            let counts: Vec<_> = game
                .subsets
                .iter()
                .map(|subset| (subset.red, subset.green, subset.blue))
                .collect();
            let (red_counts, green_counts, blue_counts): (Vec<_>, Vec<_>, Vec<_>) =
                multiunzip(counts);

            let min_red = red_counts.into_iter().max();
            let min_green = blue_counts.into_iter().max();
            let min_blue = green_counts.into_iter().max();

            min_red
                .and_then(|red| min_green.map(|green| red * green))
                .and_then(|total| min_blue.map(|blue| total * blue))
        })
        .sum();

    let power_total = power_total.ok_or_else(|| anyhow!("no maximum cube count found"))?;

    println!("Part two answer: {power_total}");

    Ok(())
}

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

#[derive(Debug)]
struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    map(
        separated_pair(
            preceded(tag("Game "), map_res(digit1, str::parse)),
            tag(": "),
            separated_list0(tag("; "), parse_subset),
        ),
        |(id, subsets)| Game { id, subsets },
    )(input)
}

fn parse_subset(input: &str) -> IResult<&str, Subset> {
    map(
        separated_list1(
            tag(", "),
            alt((
                map(
                    terminated(map_res(digit1, str::parse), tag(" red")),
                    Color::Red,
                ),
                map(
                    terminated(map_res(digit1, str::parse), tag(" green")),
                    Color::Green,
                ),
                map(
                    terminated(map_res(digit1, str::parse), tag(" blue")),
                    Color::Blue,
                ),
            )),
        ),
        |colors| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for color in colors {
                match color {
                    Color::Red(count) => red += count,
                    Color::Green(count) => green += count,
                    Color::Blue(count) => blue += count,
                }
            }

            Subset { red, green, blue }
        },
    )(input)
}
