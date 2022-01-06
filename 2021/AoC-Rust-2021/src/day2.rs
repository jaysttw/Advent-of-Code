use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Position {
    x: i32, // horizontal position
    depth: i32, // positive indicates deeper
}

fn position_from_str_tuple(input: (&str, &str)) -> Position {
    let direction: &str = input.0;
    let value: i32 = input.1.parse::<i32>().unwrap();

    match direction {
        "forward" => Position { value, 0 }
        "down" => Position { 0, value }
        "up" => Position { 0, -value }
        _ => panic!("tuple not recognised: {:?}, {:?}", direction, value)
    }
}

#[aoc_generator(day2)]
fn day2_input(input: &str) -> Vec<i32> {
    // consume the input, split out into lines
    // split each line into 
    input.lines().map(|l| {
        l.split(' ').collect_tuple(2).unwrap().map(|t| position_from_str_tuple(t))
    }).collect::<Vec<Position>>
}

#[aoc(day2)]
fn day2(input: &[Position]) -> i32 {
    
}