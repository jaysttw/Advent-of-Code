use aoc_runner_derive::aoc;
use std::collections::HashSet;

fn find_distinct_characters(input: &str, n: usize) -> usize {
    let cs: Vec<char> = input.chars().collect();

    let truth_vec: Vec<bool> = cs
    .windows(n)
    .map(|x| HashSet::<&char>::from_iter(x.iter()).len() == n)
    .collect();

    truth_vec.iter().position(|&x| x == true).expect("No unique match found!") + n
}

#[aoc(day6, part1, mine)]
fn part1(input: &str) -> usize {
    find_distinct_characters(input, 4)
}

#[aoc(day6, part2, mine)]
fn part2(input: &str) -> usize {
    find_distinct_characters(input, 14)
}