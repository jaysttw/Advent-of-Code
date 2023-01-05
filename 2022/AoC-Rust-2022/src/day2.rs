use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;
// use std::error::Error;
// use itertools::Itertools;

#[derive(PartialEq, PartialOrd)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct InputError {
    culprit: String,
}

impl InputError {
    fn new(msg: &str) -> InputError {
        InputError{culprit: msg.to_string()}
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected input: {}", self.culprit)
    }
}

struct Round(Play, Play);

impl Round {
    fn hand_score(&self) -> i32 {
        match &self.1 {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
            _ => panic!("Unidentified type in round!"),
        }
    }

    fn round_score(&self) -> i32 {
        match &self.1 {
            Play::Scissors => match &self.0 {
                Play::Rock => 0,
                Play::Paper => 6,
                Play::Scissors => 3,
                _ => panic!("Unidentified type in round for other!"),
            },
            Play::Rock | Play::Paper => {
                if &self.1 > &self.0 {
                    6
                } else if &self.1 == &self.0 {
                    3
                } else {
                    0
                }
            }
            _ => panic!("Cannot calculate round score: unidentified self type"),
        }
    }

    fn score(&self) -> i32 {
        self.hand_score() + self.round_score()
    }
}

fn string_to_round(line: &str) -> Round {
    line.trim().to_string();

    if line.len() != 3 {
        panic!("Unexpected input! Input: {:?}", line)
    };

    let other: Result<Play, InputError> = match line.chars().nth(0).unwrap() {
        'A' => Ok(Play::Rock),
        'B' => Ok(Play::Paper),
        'C' => Ok(Play::Scissors),
        _ => Err(InputError::new(&line.chars().nth(0).unwrap().to_string())),
    };
    let own: Result<Play, InputError> = match line.chars().nth(2).unwrap() {
        'X' => Ok(Play::Rock),
        'Y' => Ok(Play::Paper),
        'Z' => Ok(Play::Scissors),
        _ => Err(InputError::new(&line.chars().nth(2).unwrap().to_string())),
    };

    let rother: Play = other.unwrap();
    let rown: Play = own.unwrap();

    Round(rother, rown)
}

#[aoc_generator(day2)]
fn part1_input(input: &str) -> Vec<Round> {
    // should panic if unexpected input found
    let vec_input: Vec<Round> = input
        .lines()
        .map(|l| string_to_round(l))
        .collect();

    vec_input
}

#[aoc(day2, part1, mine)]
fn part1(input: &[Round]) -> i32 {
    // let result = input.iter().map(|v| v.iter().sum()).collect::<Vec<i32>>();
    // sum_of_n_highest(&result, 1)
    // result.into_iter().max().unwrap()
    let result = input
        .iter()
        .map(|r| r.score())
        .collect::<Vec<i32>>()
        .into_iter()
        .sum();
    result
}

// #[aoc(day2, part2, mine)]
// fn part2(input: &[Vec<Round>]) -> i32 {
// let result = input.iter().map(|v| v.iter().sum()).collect::<Vec<i32>>();
// sum_of_n_highest(&result, 3)
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part1_first() {
    //     assert_eq!(part1(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 7)
    // }
}
