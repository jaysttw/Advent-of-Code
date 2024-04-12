use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;
use std::num::ParseIntError;

use crate::utils::errors;

#[derive(Debug)]
enum InputErrors {
    Length(errors::InputLengthError),
    ParseInt(ParseIntError),
    UnrecognisedInput(UnrecognisedInputError)
}

impl From<ParseIntError> for InputErrors {
    fn from(err: ParseIntError) -> InputErrors {
        InputErrors::ParseInt(err)
    }
}

impl From<UnrecognisedInputError> for InputErrors {
    fn from(err: UnrecognisedInputError) -> InputErrors {
        InputErrors::UnrecognisedInput(err)
    }
}

#[derive(Debug)]
struct UnrecognisedInputError {
    input: String,
}

impl fmt::Display for UnrecognisedInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unrecognised instruction: {}",
            self.input,
        )
    }
}

impl UnrecognisedInputError {
    fn new(input: String) -> UnrecognisedInputError {
        UnrecognisedInputError { input : input }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    NoOp,
    AddX(isize),
}

impl Instruction {
    fn from_string(input: &str) -> Result<Instruction, InputErrors> {
        let in_str: Vec<&str> = input.split(' ').collect();
        if in_str.len() == 1 {
            if in_str[0] == "noop" {
                Ok(Instruction::NoOp)
            } else {
                Err(
                    InputErrors::UnrecognisedInput(
                        UnrecognisedInputError::new(input.to_string())
                    )
                )
            }
        } else if in_str.len() == 2 {
            if in_str[0] == "addx" {
                Ok(Instruction::AddX(in_str[1].parse::<isize>()?))
            } else {
                Err(
                    InputErrors::UnrecognisedInput(
                        UnrecognisedInputError::new(input.to_string())
                    )
                )
            }
        } else {
            Err(
                InputErrors::Length(
                    errors::InputLengthError::new(
                        1,
                        in_str.len(),
                        input.to_string(),
                    )
                )
            )
        }
    }
}
#[derive(Clone)]
struct Programme {
    cycles: Vec<usize>,
    register: Vec<isize>,
}

impl Programme {
    fn from_instructions(inst_vec: Vec<Instruction>) -> Programme {
        let pairs: Vec<(usize, isize)> = inst_vec
        .iter()
        .map(
            |x| Self::pair_from_instruction(*x)
        ).collect();

        let (cycle_counts, register_moves): (Vec<usize>, Vec<isize>) = pairs
        .iter()
        .cloned()
        .unzip();

        let cycles: Vec<usize> = cycle_counts
        .into_iter()
        .scan(
            0,
            |acc, x| {
                *acc += x;
                Some(*acc)
            }
        ).collect();

        let register: Vec<isize> = register_moves
        .into_iter()
        .scan(
            1,
            |acc, x| {
                *acc += x;
                Some(*acc)
            }
        ).collect();

        Programme {
            cycles : cycles,
            register : register,
        }
    }

    fn pair_from_instruction(inst: Instruction) -> (usize, isize) {
        let cycle: usize = match inst {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        };

        let reg: isize = match inst {
            Instruction::NoOp => 0,
            Instruction::AddX(x) => x,
        };
        
        (cycle, reg)
    }

    fn check_register(self, cycle: usize) -> isize {
        // cycles should be sorted
        let pos: usize = match self.cycles.binary_search(&(cycle - 1)) {
            Ok(n) => n,
            Err(n) => n-1,
        };

        // println!("Signal at {} is {}", cycle, self.register[pos]);

        self.register[pos]
    }

    fn signal_strength(self) -> isize {
        let length: usize = *self.cycles.iter().max().unwrap();
        let intervals: Vec<usize> = (20..length).step_by(40).collect();

        let signals: Vec<isize> = intervals
        .iter()
        .map(
            |x| *x as isize * <Programme as Clone>::clone(&self).check_register(*x)
        ).collect();

        // println!("Signal strengths: {:?}", signals);

        signals.iter().sum()
    }
}

#[aoc_generator(day10)]
fn part1_input(input: &str) -> Vec<Instruction> {
    input.lines().map(
        |l| Instruction::from_string(l).unwrap()
    ).collect::<Vec<Instruction>>()
}

#[aoc(day10, part1, mine)]
fn part1(input: &[Instruction]) -> isize {
    let sample_programme: Programme = generate_sample1();

    // println!("Sample programme cycles: {:?}", sample_programme.cycles);
    // println!("Sample programme registers: {:?}", sample_programme.register);

    println!("Sample signal strength: {}", sample_programme.signal_strength());

    let prog: Programme = Programme::from_instructions(input.to_vec());

    prog.signal_strength()
}

fn generate_sample1() -> Programme {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    let sample_instructions: Vec<Instruction> = input
    .lines()
    .map(
        |l| Instruction::from_string(l).unwrap()
    ).collect();

    Programme::from_instructions(sample_instructions)
}