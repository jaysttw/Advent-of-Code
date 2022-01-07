use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone)]
struct PositionMessage {
    x: i32, // horizontal position
    depth: i32, // positive indicates deeper
}

impl Add for PositionMessage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        PositionMessage { x: self.x + other.x, depth: self.depth + other.depth }
    }
}

impl AddAssign for PositionMessage {
    fn add_assign(&mut self, other: Self) {
        *self = PositionMessage {
            x: self.x + other.x,
            depth: self.depth + other.depth,
        };
    }
}

impl Sub for PositionMessage {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        PositionMessage { x: self.x - other.x, depth: self.depth - other.depth }
    }
}

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    aim: i32,
    depth: i32,
}

impl Position {
    fn area(&self) -> i32 {
        self.x * self.depth
    }

    fn consume_message(&self, input: PositionMessage) -> Position {
        let new_x: i32 = self.x + input.x;
        let new_aim: i32 = self.aim + input.depth;
        Position {
            x: self.x + input.x,
            aim: new_aim,
            depth: self.depth + (input.x * self.aim),
        }
    }
}

fn position_from_str(input: &str) -> PositionMessage {
    let split: (&str, &str) = input.split(' ').collect_tuple::<(_,_)>().unwrap();
    let direction: &str = split.0;
    let value: i32 = split.1.parse::<i32>().unwrap();

    match direction {
        "forward" => PositionMessage { x: value, depth: 0 },
        "down" => PositionMessage { x: 0, depth: value },
        "up" => PositionMessage { x: 0, depth: -value },
        _ => panic!("tuple not recognised: {:?}, {:?}", direction, value),
    }
}

#[aoc_generator(day2)]
fn day2_input(input: &str) -> Vec<PositionMessage> {
    // consume the input, split out into lines
    // split each line into 
    input.lines().map(|l| {
        position_from_str(l)
    }).collect::<Vec<PositionMessage>>()
}

#[aoc(day2, part1)]
fn day2_part1(input: &[PositionMessage]) -> i32 {
    let mut final_position: PositionMessage = PositionMessage { x: 0, depth: 0};
    for pos in input {
        final_position += *pos;
    }
    final_position.x * final_position.depth
}

#[aoc(day2, part2)]
fn day2_part2(input: &[PositionMessage]) -> i32 {
    let mut final_position = Position { x: 0, aim: 0, depth: 0 };
    for pos in input {
        final_position = final_position.consume_message(*pos);
    }
    final_position.area()
}