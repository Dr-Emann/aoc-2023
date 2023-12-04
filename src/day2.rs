use aoc_runner_derive::{aoc, aoc_generator};
use smallvec::SmallVec;
use std::mem;
use winnow::ascii::newline;
use winnow::combinator::{alt, fail, opt, preceded, separated, separated_pair, terminated};
use winnow::error::{ErrMode, StrContext};
use winnow::stream::{Accumulate, Stream};
use winnow::{ascii, PResult, Parser};

const GAME_INLINE_SIZE: usize = (mem::size_of::<usize>() * 3) / mem::size_of::<Set>();

#[derive(Debug)]
struct Game {
    sets: SmallVec<[Set; GAME_INLINE_SIZE]>,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
struct Set {
    blue: u8,
    red: u8,
    green: u8,
}

enum Color {
    Blue,
    Red,
    Green,
}

fn parse_game_name(input: &mut &str) -> PResult<()> {
    preceded("Game ", ascii::digit1)
        .map(|_| ())
        .parse_next(input)
}

fn parse_game(input: &mut &str) -> PResult<Game> {
    let _ = preceded(parse_game_name, ": ").parse_next(input)?;
    let SmallVecAccumulator(sets) = separated(1.., parse_set, "; ").parse_next(input)?;
    Ok(Game { sets })
}

fn parse_color(input: &mut &str) -> PResult<Color> {
    alt((
        "blue".map(|_| Color::Blue),
        "red".map(|_| Color::Red),
        "green".map(|_| Color::Green),
        fail,
    ))
    .context(StrContext::Label("color"))
    .parse_next(input)
}

fn parse_count_and_color(input: &mut &str) -> PResult<(u8, Color)> {
    separated_pair(ascii::dec_uint, ' ', parse_color).parse_next(input)
}

fn parse_set(input: &mut &str) -> PResult<Set> {
    let mut set = Set::default();

    let mut apply_count_and_color = |(count, color): (u8, Color)| match color {
        Color::Blue => set.blue = count,
        Color::Red => set.red = count,
        Color::Green => set.green = count,
    };
    // always at least one count and color
    let (count, color) = parse_count_and_color(input)?;
    apply_count_and_color((count, color));

    loop {
        let start = input.checkpoint();
        match preceded(", ", parse_count_and_color).parse_next(input) {
            Ok((count, color)) => apply_count_and_color((count, color)),
            Err(ErrMode::Backtrack(_)) => {
                input.reset(start);
                break;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(set)
}

pub struct SmallVecAccumulator<T, const N: usize>(SmallVec<[T; N]>);

impl<T, const N: usize> Accumulate<T> for SmallVecAccumulator<T, N> {
    fn initial(capacity: Option<usize>) -> Self {
        Self(SmallVec::with_capacity(capacity.unwrap_or(N)))
    }

    fn accumulate(&mut self, acc: T) {
        self.0.push(acc);
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    terminated(separated(1.., parse_game, newline), opt(newline))
        .parse(input)
        .unwrap()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> u32 {
    const MAX_RED: u8 = 12;
    const MAX_GREEN: u8 = 13;
    const MAX_BLUE: u8 = 14;

    let mut id_sum = 0;
    for (i, game) in input.iter().enumerate() {
        if game
            .sets
            .iter()
            .all(|set| set.red <= MAX_RED && set.green <= MAX_GREEN && set.blue <= MAX_BLUE)
        {
            id_sum += i as u32 + 1;
        }
    }
    id_sum
}

fn power(set: Set) -> u32 {
    u32::from(set.blue) * u32::from(set.red) * u32::from(set.green)
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> u32 {
    let mut total_power = 0;
    for game in input {
        let mut min_set = Set::default();
        for set in &game.sets {
            min_set.blue = min_set.blue.max(set.blue);
            min_set.red = min_set.red.max(set.red);
            min_set.green = min_set.green.max(set.green);
        }
        total_power += power(min_set);
    }
    total_power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        assert_eq!(
            parse_set.parse("1 blue").unwrap(),
            Set {
                blue: 1,
                ..Default::default()
            }
        );
        assert_eq!(
            parse_set.parse("1 blue, 2 red, 3 green").unwrap(),
            Set {
                blue: 1,
                red: 2,
                green: 3,
            }
        );
    }

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 2286);
    }
}
