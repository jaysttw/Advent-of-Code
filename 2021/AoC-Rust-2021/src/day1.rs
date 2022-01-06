use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn part1_input(input: &str) -> Vec<i32> {
    // should panic if unexpected input found
    input.lines().map(|l| {
        l.parse::<i32>().unwrap()
    }).collect::<Vec<i32>>()
}

#[aoc(day1, part1, mine)]
fn part1(input: &[i32]) -> i32 {
    // https://users.rust-lang.org/t/difference-between-elements-in-vec-f64/16965
    let diff_vec: Vec<i32> = input.iter().tuple_windows::<(_,_)>().map(|w| w.1 - w.0).collect::<Vec<i32>>();
    let result: i32 = diff_vec.into_iter().map(|x| {
        if x > 0 { 1 } else { 0 }
    }).sum();
    result
}

#[aoc(day1, part1, rbtcollins)]
fn part1_rbt(input: &[i32]) -> usize {
    input
        .iter()
        .tuple_windows::<(_,_)>()
        .filter(|window| window.0 < window.1)
        .count()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> usize {
    input.iter().tuple_windows::<(_,_,_,_)>().filter(|w| w.0 < w.3).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1_first() {
        assert_eq!(part1(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 7)
    }
}