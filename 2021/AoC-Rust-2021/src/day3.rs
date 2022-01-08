use aoc_runner_derive::{aoc, aoc_generator};
// use itertools::Itertools;
use std::str;

#[aoc_generator(day3, part1)]
fn day3_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_owned()).collect() // need to map to bits
}

#[aoc(day3, part1)]
fn day3_part1(input: &[Vec<u8>]) -> i32 {
    // with heavy borrowing from https://github.com/rbtcollins/aoc-2021/blob/main/src/day3.rs
    let mut current_count: Vec<i8> = vec![0; input[0].len()];
    for line in input {
        for bit in 0..line.len() {
            let val: i8 = (line[bit]).try_into().unwrap();
            current_count[bit] = current_count[bit] + (val * 2) - 1; // adds 1 if bit is 1, 0 otherwise.
        }
    }

    println!("current_count: {:?}", current_count);

    let mut epsilon = 0;
    let mut gamma = 0;
    for bit in current_count {
        gamma <<= 1;
        epsilon <<= 1;
        match bit {
            i if i < 0 => epsilon += 1,
            i if i > 0 => gamma += 1,
            _ => panic!("Error in matching bit {:?}.", bit)
        }
    }

    gamma * epsilon
}