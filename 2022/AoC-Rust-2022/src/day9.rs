use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;
use std::num::ParseIntError;

use crate::utils::errors;

#[derive(Debug)]
enum InputErrors {
    Direction(DirectionInputError),
    Length(errors::InputLengthError),
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for InputErrors {
    fn from(err: ParseIntError) -> InputErrors {
        InputErrors::ParseInt(err)
    }
}

#[derive(Debug)]
struct DirectionInputError {
    input: String,
}

impl fmt::Display for DirectionInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Expected direction to be 'U', 'D', 'L', 'R', received {}",
            self.input,
        )
    }
}

impl DirectionInputError {
    fn new(input: String) -> DirectionInputError {
        DirectionInputError { input : input }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Step {
    direction: Direction,
    steps: usize,
}

impl Step {
    fn from_input(line: &str) -> Result<Step, InputErrors> {
        if line.split(' ').collect::<Vec<&str>>().len() != 2 {
            return Err(
                InputErrors::Length(
                    errors::InputLengthError::new(
                        2,
                        line.split(' ').collect::<Vec<&str>>().len(),
                        line.to_string(),
                    )
                )
            )
        }

        let parts: Vec<&str> = line.split(' ').collect();

        let direction_char: char = if parts[0].len() == 1 {
            Ok(parts[0].chars().next())
        } else {
            Err(
                InputErrors::Length(
                    errors::InputLengthError::new(
                        1,
                        parts[0].len(),
                        parts[0].to_string(),
                    )
                )
            )
        }.unwrap().expect("Not a char!");

        let direction: Result<Direction, InputErrors> = match direction_char {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(
                    InputErrors::Direction(
                    DirectionInputError::new(parts[0].to_string())
                ),
            )
        };

        let steps: usize = parts[1].parse()?;

        Ok(
            Step {
                direction : direction?,
                steps : steps,
            }
        )
    }

    fn apply_to_grid(self, grid: Grid) -> Grid {
        grid.update_head(self)
    }
}

struct Grid {
    head: (isize, isize),
    tail: (isize, isize),
    tail_history: (Vec<isize>, Vec<isize>),
}

impl Grid {
    fn new() -> Grid {
        Grid {
            head : (0,0),
            tail : (0,0),
            tail_history: (vec![0], vec![0]),
        }
    }

    fn update_head(self, step: Step) -> Grid {
        let mut result: Grid = self;
        for _ in 0..step.steps {
            let new_head = match step.direction {
                Direction::Up => (result.head.0, result.head.1 + 1),
                Direction::Down => (result.head.0, result.head.1 - 1),
                Direction::Left => (result.head.0 - 1, result.head.1),
                Direction::Right => (result.head.0 + 1, result.head.1),
            };
            println!(
                "Current head: {:?}, new head: {:?}",
                result.head,
                new_head
            );

            // println!("Current tail history: {:?}", result.tail_history);

            result = result.update_tail(new_head);
        }

        result
    }

    fn update_tail(self, new_head: (isize, isize)) -> Grid {
        if new_head.0.abs_diff(self.tail.0) <= 1 && 
        new_head.1.abs_diff(self.tail.1) <= 1 {
            // Assumes that we're only counting individual visits to the spot,
            // not staying on the spot for example.
            return Grid {
                head : new_head,
                tail : self.tail.clone(),
                tail_history : self.tail_history.clone(),
            }
        } else if new_head.0.abs_diff(self.tail.0) == 2 && 
        new_head.1.abs_diff(self.tail.1) <= 1 {
            // handle case if x-axis
            let new_one: isize = if new_head.1.abs_diff(self.tail.1) == 1 {
                if new_head.1 > self.tail.1 {
                    self.tail.1 + 1
                } else {
                    self.tail.1 - 1
                }
            } else { self.tail.1 };
            let new_tail: (isize, isize) = if new_head.0 < self.tail.0 {
                (self.tail.0 - 1, new_one)
            } else {
                (self.tail.0 + 1, new_one)
            };

            let new_tail_history: (Vec<isize>, Vec<isize>) = Self::update_tail_history(self.tail_history, new_tail);

            return Grid {
                head : new_head,
                tail : new_tail, 
                tail_history : new_tail_history,
            }
        } else if new_head.0.abs_diff(self.tail.0) <= 1 && 
        new_head.1.abs_diff(self.tail.1) == 2 {
            // handle case if y-axis
            let new_zero: isize = if new_head.0.abs_diff(self.tail.0) == 1 {
                if new_head.0 > self.tail.0 {
                    self.tail.0 + 1
                } else {
                    self.tail.0 - 1
                }
            } else { self.tail.0 };
            let new_tail: (isize, isize) = if new_head.1 < self.tail.1 {
                (new_zero, self.tail.1 - 1)
            } else {
                (new_zero, self.tail.1 + 1)
            };

            let new_tail_history: (Vec<isize>, Vec<isize>) = Self::update_tail_history(self.tail_history, new_tail);

            return Grid {
                head : new_head,
                tail : new_tail, 
                tail_history : new_tail_history,
            }
        } else {
            panic!(
                "Head moved out of bounds in previous step! Head: {:?}, tail: {:?}",
                new_head,
                self.tail,
            )
        }
    }

    fn update_tail_history(cur: (Vec<isize>, Vec<isize>), new_tail: (isize, isize)) -> (Vec<isize>, Vec<isize>) {
        let zero: Vec<isize> = [cur.0.as_slice(), vec![new_tail.0].as_slice()]
        .concat()
        .to_vec();
        let one: Vec<isize> = [cur.1.as_slice(), vec![new_tail.1].as_slice()].
        concat()
        .to_vec();
        (zero, one)
    }
}

struct GridHistory {
    history: (Vec<isize>, Vec<isize>),
    grid: Vec<Vec<usize>>,
}

impl GridHistory {
    fn from_history(history: (Vec<isize>, Vec<isize>)) -> GridHistory {
        let x_min: isize = *history.0.iter().min().unwrap();
        let x_max: isize = *history.0.iter().max().unwrap();
        let y_min: isize = *history.1.iter().min().unwrap();
        let y_max: isize = *history.1.iter().max().unwrap();

        let x_offset: isize = 0isize - x_min;
        let y_offset: isize = 0isize - y_min;

        let x_range: usize = (x_max - x_min + 1)
        .try_into()
        .unwrap_or_else(|_| panic!("x_max ({}) < x_min ({})", x_max, x_min));
        let y_range: usize = (y_max - y_min + 1)
        .try_into()
        .unwrap_or_else(|_| panic!("y_max ({}) < y_min ({})", y_max, y_min));

        let mut grid: Vec<Vec<usize>> = vec![vec![0usize; y_range]; x_range];

        for (x,y) in history.0.iter().zip(history.1.iter()) {
            let x_pos: usize = (x + x_offset).try_into().unwrap();
            let y_pos: usize = (y + y_offset).try_into().unwrap();
            grid[x_pos][y_pos] += 1;
        }

        GridHistory {
            history: history,
            grid: grid,
        }
    }
}

fn generate_sample() -> Vec<Step> {
    let input: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let result: Vec<Step> = input
    .lines()
    .map(|l| Step::from_input(l).unwrap())
    .collect();

    result
}

#[aoc_generator(day9)]
fn part1_input(input: &str) -> Vec<Step> {
    let result: Vec<Step> = input
    .lines()
    .map(|l| Step::from_input(l).unwrap())
    .collect();

    result
}

#[aoc(day9, part1, mine)]
fn part1(steps: &[Step]) -> usize {
    // code for sample
    let sample_steps: Vec<Step> = generate_sample();

    let sample_result: Grid = sample_steps
    .iter()
    .fold(
        Grid::new(),
        |acc, i| i.apply_to_grid(acc)
    );

    let sample_history: GridHistory = GridHistory::from_history(
        sample_result.tail_history
    );

    let sample_visited: usize = sample_history.grid
    .iter()
    .map(
        |v| v.iter()
        .map(|i| *i != 0)
        .collect::<Vec<bool>>()
        .iter()
        .filter(|x| **x)
        .count()
    )
    .sum();

    println!("Total positions visited in sample: {}", sample_visited);

    // main body
    let result_grid: Grid = steps
    .iter()
    .fold(
        Grid::new(),
        |acc, i| i.apply_to_grid(acc)
    );

    let result_history: GridHistory = GridHistory::from_history(
        result_grid.tail_history
    );

    result_history.grid
    .iter()
    .map(
        |v| v.iter()
        .map(|i| *i != 0)
        .collect::<Vec<bool>>()
        .iter()
        .filter(|x| **x)
        .count()
    )
    .sum()
}