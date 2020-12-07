use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use itertools::Itertools;

#[aoc_generator(day1, part1)]
fn part1_input(input: &str) -> Result<Combinations<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect().combinations(2)
}

#[aoc(day, part1)]
fn part1(input: Vec<Vec<i32>>) -> i32 {
    // iterate through the input to find the pair that sums to 2020
    for i in input {
        if i[0] + i[1] == 2020 {
            return i[0] * i[1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1_first() {
        assert_eq!(part1(&[1771, 1993, 1840, 942, 10, 180]), 1840*180)
        assert_eq!(part1(&[1771, 1993, 140, 942, 110, 27, 180]), 1993*27))
    }
}