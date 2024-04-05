use aoc_runner_derive::{aoc, aoc_generator};
use crate::utils::vecvec;

struct Grid {
    grid: Vec<Vec<usize>>,
    rotated: Vec<Vec<usize>>,
    // hidden: Vec<Vec<bool>>,
}

impl Grid {
    fn from_input(input: Vec<Vec<usize>>) -> Grid {
        let rotated: Vec<Vec<usize>> = vecvec::rotate_vecvec(
            input.clone(),
            true,
        ).unwrap();
        // .unwrap_or_else(|e|) panic!("{:?}", e);

        Grid { grid : input, rotated : rotated }
    }

    fn calculate_hidden(&self) -> usize {
        let checked_grid: Vec<Vec<bool>> = self.grid.iter().map(|r| check_row(r)).collect::<Vec<Vec<bool>>>();
        let checked_rotated: Vec<Vec<bool>> = vecvec::rotate_vecvec(
            self.rotated.iter().map(|r| check_row(r)).collect::<Vec<Vec<bool>>>(),
            false,
        ).unwrap();
        // .unwrap_or_else(|e| panic!("{:?}", e));

        // println!("Checked grid: {:?}", checked_grid);
        // println!("Rotated grid: {:?}", checked_rotated);

        let result: usize = if checked_grid.iter().map(|i| i.len()).collect::<Vec<usize>>() == checked_rotated.iter().map(|i| i.len()).collect::<Vec<usize>>() {
            // grids match in dimensions
            // let result: Vec<Vec<bool>> = checked_grid
            // .enumerate()
            // .map(
            //     |i, r| r.enumerate().map(
            //         |x, s| s && checked_rotated[i][x]
            //     ).collect::<Vec<bool>>()
            // ).collect();
            
            let visible: Vec<Vec<bool>> = checked_grid
            .iter()
            .enumerate()
            .map(
                |(i, r)| r.iter().zip(checked_rotated[i].iter()).map(
                    |(a, b)| !(*a && *b)
                ).collect::<Vec<bool>>()
            ).collect();

            // println!("Visible grid: {:?}", visible);

            visible
            .iter()
            .map(
                |r| r.iter()
                .filter(|item| **item)
                .map(|x| *x)
                .collect::<Vec<bool>>().len()
            ).sum()
        } else {
            panic!("Checked grids are not the same shape!");
        };

        result
    }
}

fn check_row(row: &Vec<usize>) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];

    let row_length = row.len();

    for idx in 0..row_length {
        // println!("Row index: {} of {}", idx, row_length);
        if idx == 0 || idx == row_length - 1 {
            result.push(false);
        } else {
            let left: Vec<usize> = row[0..idx].to_vec();
            let right: Vec<usize> = row[idx+1..row_length].to_vec();

            // println!("Index {} Left max: {}", idx, *left.iter().max().unwrap(),);
            // println!("Index {} Right max: {}", idx, *right.iter().max().unwrap(),);

            if row[idx] <= *left.iter().max().unwrap() && row[idx] <= *right.iter().max().unwrap() {
                result.push(true);
            } else {
                result.push(false);
            }
        }
    }

    result
}

#[aoc_generator(day8)]
fn part1_input(input: &str) -> Grid {
    let grid_vecs: Vec<Vec<usize>> = input
    .lines()
    .map(
        |l| l.chars().map(
            |c| usize::try_from(c.to_digit(10).unwrap()).unwrap()
        ).collect::<Vec<usize>>()
    )
    .collect();

    Grid::from_input(grid_vecs)
}

#[aoc(day8, part1, mine)]
fn part1(grid: &Grid) -> usize {
    let sample_input = "30373
25512
65332
33549
35390";

    let sample_grid: Grid = Grid::from_input(
        sample_input.lines()
        .map(
            |l| l.chars().map(
                |c| usize::try_from(c.to_digit(10).unwrap()).unwrap()
            ).collect::<Vec<usize>>()
        ).collect::<Vec<Vec<usize>>>()
    );

    println!("Result from sample grid: {}", sample_grid.calculate_hidden());

    grid.calculate_hidden()
}