use aoc_runner_derive::{aoc, aoc_generator};
// use std::error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Clone)]
struct Layout {
    layout: Vec<Vec<char>>,
}

impl Layout {
    fn top(&self) -> String {
        let top: String = self.layout.iter().map(|v| v[v.len() - 1]).collect();
        top
    }

    fn from_input(input: &str) -> Result<Layout, InputErrors> {
        let raw_vectors: Vec<Vec<char>> = input.lines().map(|l| l.chars().skip(1).step_by(4).collect::<Vec<char>>()).collect(); // skips 1 instead of 5 since the example shows only 1 char space.

        let initial_stack_height: usize = raw_vectors.len() - 1;
        let stack_row: Vec<char> = raw_vectors[initial_stack_height].clone();
        let stack_count: usize = stack_row.len();
        let initial_stacks: Vec<Vec<char>> = (&raw_vectors[0..initial_stack_height+1]).to_vec();

        let maximum_initial_width = raw_vectors.iter().map(|v| v.len()).max().unwrap(); // using unwrap here because there should always be a value?

        if maximum_initial_width > stack_count {
            // checking that the input is correctly sized
            return Err(
                InputErrors::Length(
                    InputLengthError::new(
                        stack_count,
                        maximum_initial_width,
                        format!("{:?}", raw_vectors),
                    )
                )
            )
        }

        let new_stacks: Vec<Vec<char>> = (0..stack_count)
            .map(
                |i| initial_stacks
                .iter()
                .rev()
                .map(|inner| inner[i].clone())
                .filter(|c| !c.is_whitespace()) // filter out the whitespace, won't work if stack is incorrectly formatted.
                .collect::<Vec<char>>()
            )
            .collect();

        Ok( Layout { layout: new_stacks } )
    }

    fn apply_instruction(&self, instructions: Instruction, variant: bool) -> Result<Layout, InputErrors> {
        let origin_vector_size = &self.layout[instructions.src - 1].len() - instructions.blocks;

        // implement a check for vector length to make sure that drain can proceed.
        if origin_vector_size <= 0 {
            return Err(
                InputErrors::Length(
                    InputLengthError::new(
                        origin_vector_size,
                        self.layout[instructions.src - 1].len(),
                        format!("{:?}", &self.layout),
                    )
                )
            );
        }

        let mut new_layout: Vec<Vec<char>> = self.layout.clone();
        let mut moved: Vec<char> = if variant {
            new_layout[instructions.src - 1].drain(origin_vector_size..).collect()
        } else {
            new_layout[instructions.src - 1].drain(origin_vector_size..).rev().collect()
        };
        new_layout[instructions.dest - 1].append(&mut moved);

        Ok(Layout { layout: new_layout.clone() })
    }
}

struct Instruction {
    blocks: usize,
    src: usize,
    dest: usize,
}

impl Instruction {
    fn from_input(input: &str) -> Result<Instruction, InputErrors> {
        // "move x from y to z"
        // assumes that it's 6 blocks long.

        let vector: Vec<&str> = input.split(" ").collect();

        if vector.len() != 6 {
            return Err(
                InputErrors::Length(
                    InputLengthError::new(
                        6,
                        vector.len(),
                        format!("{:?}", vector),
                    )
                )
            )
        }

        // not sure if these will bubble up properly or result in a type mismatch.
        let x: usize = vector[1].parse()?;
        let y: usize = vector[3].parse()?;
        let z: usize = vector[5].parse()?;

        Ok(Instruction { blocks: x, src: y, dest: z })
    }
    fn apply_to_layout(&self, stack: Layout, variant: bool) -> Result<Layout, InputErrors> {
        // let stack = match stack_in {
        //     Err(e) => return Err(e),
        //     Ok(s) => s,
        // };

        let origin_vector_size = stack.layout[&self.src - 1].len() - self.blocks;

        if origin_vector_size <= 0 {
            return Err(
                InputErrors::Length(
                    InputLengthError::new(
                        origin_vector_size,
                        stack.layout[&self.src - 1].len(),
                        format!("{:?}", stack.layout),
                    )
                )
            )
        }

        let mut new_layout: Vec<Vec<char>> = stack.layout.clone();
        let mut moved: Vec<char> = if variant {
            new_layout[self.src - 1].drain(origin_vector_size..).collect()
        } else {
            new_layout[self.src - 1].drain(origin_vector_size..).rev().collect()
        };
        new_layout[&self.dest - 1].append(&mut moved);

        Ok(Layout { layout: new_layout.clone() })
    }
}

struct Inputs {
    layout: Layout,
    instructions: Vec<Instruction>,
}

fn convert_inputs(input: Vec<&str>) -> Inputs {
    // made a design decision to panic if errors are encountered, since there's no reasonable way to continue.

    if input.len() != 2 {
        panic!(
            "{:?}",
                InputLengthError::new(
                2,
                input.len(),
                format!("{:?}", input),
            )
        );
    };

    let layout: Layout = Layout::from_input(input[0]).unwrap();
    let instructions: Vec<Instruction> = input[1].lines().map(|l| Instruction::from_input(l).unwrap()).collect();

    Inputs {layout: layout, instructions: instructions}
}

#[derive(Debug)]
enum InputErrors {
    Length(InputLengthError),
    ParseError(ParseIntError),
}

impl From<ParseIntError> for InputErrors {
    fn from(err: ParseIntError) -> InputErrors {
        InputErrors::ParseError(err)
    }
}

#[derive(Debug)]
struct InputLengthError {
    expected: usize,
    actual: usize,
    items: String,
}

impl fmt::Display for InputLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect vector length, expected: {}, actual: {}. Culprit: {:?}", self.expected, self.actual, self.items)
    }
}

impl InputLengthError {
    fn new(expected: usize, actual: usize, msg: String) -> InputLengthError {
        InputLengthError {
            expected: expected,
            actual: actual,
            items: msg,//.to_string(),
        }
    }
}

#[aoc_generator(day5)]
fn part1_input(input: &str) -> Inputs {
    let input_pair: Vec<&str> = input.split("\n\n").collect();

    convert_inputs(input_pair)
}

#[aoc(day5, part1, mine)]
fn part1(input: &Inputs) -> String {
    let final_stack: Layout = input.instructions
    .iter()
    .try_fold(
        input.layout.clone(),
        |acc, i| i.apply_to_layout(acc, false)
    )
    .unwrap();

    final_stack.top()
}

#[aoc(day5, part2, mine)]
fn part2(input: &Inputs) -> String {
    let final_stack: Layout = input.instructions
    .iter()
    .try_fold(
        input.layout.clone(),
        |acc, i| i.apply_to_layout(acc, true)
    )
    .unwrap();

    final_stack.top()
}