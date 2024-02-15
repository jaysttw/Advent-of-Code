use aoc_runner_derive::{aoc, aoc_generator};
use std::error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum Error {
    InputError(InputError),
    TooManyError(TooManyError),
    ParseError(ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InputError(input_error) => write!(f, "Unexpected input: {}", input_error.culprit),
            Error::TooManyError(too_many_error) => write!(f, "Too many: {}", too_many_error.items),
            Error::ParseError(parse_int_error) => write!(f, "{}", parse_int_error),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseError(err)
    }
}

impl From<InputError> for Error {
    fn from(err: InputError) -> Error {
        Error::InputError(err)
    }
}

impl From<TooManyError> for Error {
    fn from(err: TooManyError) -> Error {
        Error::TooManyError(err)
    }
}

#[derive(Debug)]
struct InputError {
    culprit: String,
}

impl InputError {
    fn new(msg: &str) -> InputError {
        InputError {
            culprit: msg.to_string(),
        }
    }
}

impl error::Error for InputError {}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected input: {}", self.culprit)
    }
}

#[derive(Debug)]
struct TooManyError {
    items: String,
}

impl TooManyError {
    fn new(msg: &str) -> TooManyError {
        TooManyError {
            items: msg.to_string(),
        }
    }
}

impl error::Error for TooManyError {}

impl fmt::Display for TooManyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Too many: {}", self.items)
    }
}

struct ElfPair {
    one_upper: i32,
    one_lower: i32,
    two_upper: i32,
    two_lower: i32,
    fully_contained: bool,
    overlapped: bool,
}

impl ElfPair {
    fn new(bounds: Vec<i32>) -> ElfPair {
        // Potentially dangerous since there isn't actually anything to guarantee the actual length.
        
        let one_upper = bounds[0];
        let one_lower = bounds[1];
        let two_upper = bounds[2];
        let two_lower = bounds[3];

        let fully_contained = if one_upper >= two_upper && one_lower <= two_lower {
            true
        } else if one_upper <= two_upper && one_lower >= two_lower {
            true
        } else {
            false
        };

        // Not checking the truth of the upper/lower (assumes that the ranges are correct)
        let overlapped = if one_upper < two_lower && one_upper < two_upper && one_lower < two_lower && one_lower < two_upper {
            false
        } else if one_lower > two_upper && one_lower > two_lower && one_upper > two_upper && one_upper > two_lower {
            false
        } else {
            true
        };

        ElfPair { one_upper: one_upper, one_lower: one_lower, two_upper: two_upper, two_lower: two_lower, fully_contained: fully_contained, overlapped: overlapped }
    }
}

fn string_to_pair(input: &str) -> Result<ElfPair, Error> {
    let elves = input.split(",");
    let bounds_result = elves.map(
        |e| e.split("-").map(|x| x.parse().map_err(|e| Error::ParseError(e))).collect::<Vec<Result<i32, Error>>>()
    )
    .collect::<Vec<Vec<Result<i32, Error>>>>()
    .into_iter()
    .flatten()
    .collect::<Result<Vec<i32>, Error>>();

    let bounds = match bounds_result {
        Ok(v) => v,
        Err(ref e) => return Err(Error::InputError(InputError::new(format!("{:?}", bounds_result).as_str())))
    };

    if bounds.len() != 4 {
        Err(Error::TooManyError(TooManyError::new(format!("{:?}", bounds).as_str())))
    } else {
        Ok(ElfPair::new(bounds))
    }
}

#[aoc_generator(day4)]
fn part1_input(input: &str) -> Vec<ElfPair> {
    let vec_input: Vec<ElfPair> = input.lines().map(|s| string_to_pair(s).unwrap()).collect();
    vec_input
}

#[aoc(day4, part1, mine)]
fn part1(input: &[ElfPair]) -> u32 {
    let total: u32 = input.into_iter().map(|ep| ep.fully_contained as u32).sum();
    total // as u32
}

#[aoc(day4, part2, mine)]
fn part2(input: &[ElfPair]) -> u32 {
    let total: u32 = input.into_iter().map(|ep| ep.overlapped as u32).sum();
    total // as u32
}