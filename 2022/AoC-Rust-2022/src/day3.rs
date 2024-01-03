use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::fmt;
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

#[derive(Clone)]
struct Rucksack {
    left: String,
    right: String,
    duplicate: Option<char>,
}

impl Rucksack {
    fn new(lstr: String, rstr: String) -> Rucksack {
        let mut new_rucksack = Rucksack {
            left: lstr,
            right: rstr,
            duplicate: None,
        };

        let duplicates = match new_rucksack.find_duplicate().unwrap() {
            Ok(v) => v,
            Err(e) => panic!("Cannot create rucksack: {:?}", e),
        };

        new_rucksack.duplicate = Some(duplicates);

        new_rucksack
    }

    fn find_duplicate(&self) -> Option<Result<char, TooManyError>> {
        // println!("Left string: {}, right string: {}", self.left, self.right);

        let left_set: HashSet<char> = HashSet::from_iter(self.left.chars());
        let right_set: HashSet<char> = HashSet::from_iter(self.right.chars());

        let intersect: HashSet<_> = left_set.intersection(&right_set).collect();

        let duplicate = match intersect.len() {
            1 => Some(Ok(intersect.into_iter().cloned().next().unwrap())),
            0 => None,
            _ => Some(Err(TooManyError::new(
                intersect.into_iter().cloned().collect::<String>().as_str(),
            ))),
        };

        duplicate
    }
}

struct ElfGroup {
    one: Rucksack,
    two: Rucksack,
    three: Rucksack,
    badge: Option<char>,
}

impl ElfGroup {
    fn new(o: Rucksack, tw: Rucksack, th: Rucksack) -> ElfGroup {
        let mut result = ElfGroup {
            one: o,
            two: tw,
            three: th,
            badge: None,
        };

        result.badge = Some(result.obtain_badge());

        result
    }

    fn obtain_badge(&self) -> char {
        // let first: HashSet<char> = HashSet::from_iter(self.one.left.chars())
        //     .union(&HashSet::from_iter(self.one.right.chars()))
        //     .collect();
        // let second: HashSet<char> = HashSet::from_iter(self.two.left.chars())
        //     .union(&HashSet::from_iter(self.two.right.chars()))
        //     .collect();
        // let third: HashSet<char> = HashSet::from_iter(self.three.left.chars())
        //     .union(&HashSet::from_iter(self.three.right.chars()))
        //     .collect();

        let first: HashSet<char> =
            HashSet::<char>::from_iter(format!("{}{}", self.one.left, self.one.right).chars());
        
        let second: HashSet<char> =
            HashSet::<char>::from_iter(format!("{}{}", self.two.left, self.two.right).chars());

        let third: HashSet<char> =
            HashSet::<char>::from_iter(format!("{}{}", self.three.left, self.three.right).chars());

        let temp_intersect: HashSet<_> = first
            .intersection(&second)
            .cloned()
            .collect::<HashSet<char>>();
        let intersect = temp_intersect
            .intersection(&third)
            .collect::<HashSet<&char>>();

        if intersect.len() != 1 {
            panic!(
                "Day 3 part 2: number of badges not equal to 1: {}",
                intersect.into_iter().cloned().collect::<String>().as_str()
            )
        } else {
            intersect.into_iter().cloned().next().unwrap()
        }
    }
}

fn string_to_rucksack(line: &str) -> Rucksack {
    let str_length: usize = line.len();
    let compartment_size: usize = str_length / 2;

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

    // if y <= 26 {
    //     Ok(y)
    // } else {
    //     Ok(y - 6)
    // }

    if y <= 26 {
        Ok(y + 26)
    } else {
        Ok(y - 32)
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

#[aoc(day3, part2, mine)]
fn part2(input: &[Rucksack]) -> i32 {
    let check_length = input.len();
    if check_length.rem_euclid(3) != 0 {
        panic!("Day 3 Part 2 input vector is wrong length ({check_length})",)
    }

    let rucksack_chunks: Vec<&[Rucksack]> = input.chunks_exact(3).collect();
    let groups: Vec<ElfGroup> = rucksack_chunks
        .into_iter()
        .map(|r| ElfGroup::new(r[0].clone(), r[1].clone(), r[2].clone()))
        .collect();

    let result = groups
        .iter()
        .map(|r| item_priority(r.badge).unwrap())
        .collect::<Vec<i32>>()
        .into_iter()
        .sum();

    // test_fn();

    result
}

fn test_fn() {
    let raw_input: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
    PmmdzqPrVvPwwTWBwg\n\
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
    ttgJtRGJQctTZtZT\n\
    CrZsJsPPZsGzwwsLwLmpwMDw";

    let input: Vec<Rucksack> = raw_input.lines().map(|l| string_to_rucksack(l)).collect();

    println!("Rucksacks created for tests.");

    let check_length = input.len();
    if check_length.rem_euclid(3) != 0 {
        panic!("Day 3 Part 2 input vector is wrong length ({check_length})",)
    }

    let rucksack_chunks: Vec<&[Rucksack]> = input.chunks_exact(3).collect();
    let groups: Vec<ElfGroup> = rucksack_chunks
        .into_iter()
        .map(|r| ElfGroup::new(r[0].clone(), r[1].clone(), r[2].clone()))
        .collect();

    let result:i32 = groups
        .iter()
        .map(|r| item_priority(r.badge).unwrap())
        .collect::<Vec<i32>>()
        .into_iter()
        .sum();

    println!("Test result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_first();
}
