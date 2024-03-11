use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

// #[aoc_generator(day6)]
// fn part1_input(input: &str) -> &str {
//     // pass directly to the function?
//     input
// }

#[aoc(day6, part1, mine)]
fn part1(input: &str) -> usize {
    let cs: Vec<char> = input.chars().collect();

    let truth_vec: Vec<bool> = cs
    .windows(4)
    .map(|x| HashSet::<&char>::from_iter(x.iter()).len() == 4)
    .collect();

    truth_vec.iter().position(|&x| x == true).expect("No unique match found!") + 4
}