use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;
use std::collections::HashSet;
// use std::error::Error;
// use itertools::Itertools;

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

impl fmt::Display for TooManyError { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Too many: {}", self.items)
    }
}

struct Rucksack {
    left: String,
    right: String,
    duplicate: Option<char>,
}

impl Rucksack {
    fn new(lstr: String, rstr: String) -> Rucksack {
        let mut new_rucksack = Rucksack{left: lstr, right: rstr, duplicate: None};

        let duplicates = match new_rucksack.find_duplicate().unwrap() {
            Ok(v) => v,
            Err(e) => panic!("Cannot create rucksack: {:?}", e)
        };

        new_rucksack.duplicate = Some(duplicates);
        
        new_rucksack
    }

    fn find_duplicate(&self) -> Option<Result<char, TooManyError>> {
        let left_set: HashSet<char> = HashSet::from_iter(self.left.chars());
        let right_set: HashSet<char> = HashSet::from_iter(self.right.chars());

        let intersect: HashSet<_> = left_set.intersection(&right_set).collect();

        let duplicate = match intersect.len() {
            1 => Some(Ok(intersect.into_iter().cloned().next().unwrap())),
            0 => None,
            _ => Some(Err(TooManyError::new(intersect.into_iter().cloned().collect::<String>().as_str()))),
        };

        duplicate
    }
}

fn string_to_rucksack(line: &str) -> Rucksack {
    let str_length: usize = line.len();
    let compartment_size: usize = str_length/2;

    let left_string = &line[0..compartment_size];
    let right_string = &line[compartment_size..];

    let left = left_string.to_string();
    let right = right_string.to_string();

    Rucksack::new(left, right)
}

fn item_priority(c: Option<char>) -> Result<i32, InputError> {
    let unwrapped_c = match c {
        Some(i) => i,
        None => return Ok(0),
    };
    
    let x = unwrapped_c as i32;
    if x < 64 || x > 122 {
        return Err(InputError::new(&c.unwrap().to_string()));
    }
    
    let y = x - 64;
    
    if y <= 26 {
        Ok(y)
    } else {
        Ok(y - 6)
    }
}

#[aoc_generator(day3)]
fn part1_input(input: &str) -> Vec<Rucksack> {
    // should panic if unexpected input found
    let vec_input: Vec<Rucksack> = input.lines().map(|l| string_to_rucksack(l)).collect();

    vec_input
}

#[aoc(day3, part1, mine)]
fn part1(input: &[Rucksack]) -> i32 {
    let result = input
        .iter()
        .map(|r| item_priority(r.duplicate).unwrap())
        .collect::<Vec<i32>>()
        .into_iter()
        .sum();
    result
}

// #[aoc(day3, part2, mine)]
// fn part2(input: &[Round]) -> i32 {
//     let result = input
//         .iter()
//         .map(|r| r.new_score())
//         .collect::<Vec<i32>>()
//         .into_iter()
//         .sum();
//     result
// }

// #[aoc(day2, part2, mine)]
// fn part2(input: &[Vec<Round>]) -> i32 {
// let result = input.iter().map(|v| v.iter().sum()).collect::<Vec<i32>>();
// sum_of_n_highest(&result, 3)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_first();
}
