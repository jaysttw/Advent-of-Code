use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::input;

// fn usize_from_enum_strln(n: usize, line: &str, sep: &str, length: usize) -> Vec<usize> {
//     let result: Vec<usize> = line
//         .split(sep)
//         .map(|i| {
//             i.parse::<usize>().expect(
//                 format!(
//                     "Could not parse {} from input line {}: {} into usize!",
//                     i,
//                     n,
//                     line,
//                 ).as_str()
//             )
//         })
//         .collect();

//     if result.len() != length {
//         panic! {
//             "Length error! Input line {}: {}, expected length: {}, found: {}",
//             n,
//             line,
//             length,
//             result.len(),
//         }
//     }

//     result
// }

fn generate_sample() -> String {
    "3   4
4   3
2   5
1   3
3   9
3   3".to_string()
}

fn generate_test_tuples() -> (Vec<usize>, Vec<usize>) {
    let test: String = generate_sample();

    let test_input = part1_input(test.as_str());
    println!("Test input: {:?}", test_input);
    let mut test_left: Vec<usize> = test_input.0.to_owned();
    let mut test_right: Vec<usize> = test_input.1.to_owned();

    test_left.sort();
    test_right.sort();

    println!("Test left: {:?}", test_left);
    println!("Test right: {:?}", test_right);

    (test_left, test_right)
}

fn individual_similarity_score(n: usize, right: Vec<usize>) -> usize {
    right.iter().filter(|&x| *x == n).count()
}

#[aoc_generator(day1)]
fn part1_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    // should panic if unexpected input found
    let pairs: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .map(|(n, l)| input::usize_from_enum_strln(n, l, "   ", 2).into_iter().collect_tuple::<(usize, usize)>().unwrap())
        .collect();

    let (left, right): (Vec<usize>, Vec<usize>) = pairs
    .iter()
    .cloned()
    .unzip();

    (left, right)
    // output
}

#[aoc(day1, part1, mine)]
fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    // test case
    let (test_left, test_right): (Vec<usize>, Vec<usize>) = generate_test_tuples();

    let test_diff: Vec<usize> = test_left.iter().zip(test_right).map(|(l, r)| l.abs_diff(r)).collect();

    println!("Test case diff vector: {:?}", test_diff);
    println!("Test case distance: {}", test_diff.into_iter().sum::<usize>());

    // actual caswe
    let mut left: Vec<usize> = input.0.to_owned();
    let mut right: Vec<usize> = input.1.to_owned();

    left.sort();
    right.sort();

    let diff: Vec<usize> = left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).collect();

    diff.into_iter().sum()
}

#[aoc(day1, part2, mine)]
fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    // test case
    let (test_left, test_right): (Vec<usize>, Vec<usize>) = generate_test_tuples();
    let test_similarity_vec: Vec<usize> = test_left
    .to_owned()
    .iter()
    .map(|i| individual_similarity_score(*i, test_right.clone()))
    .zip(test_left)
    .map(|(s, t)| s*t)
    .collect();

    println!("Test similarity vector: {:?}", test_similarity_vec);
    println!("Test similarity score: {}", test_similarity_vec.iter().sum::<usize>());

    // actual case
    let mut left: Vec<usize> = input.0.to_owned();
    let mut right: Vec<usize> = input.1.to_owned();

    left.sort();
    right.sort();

    let similarity_vec: Vec<usize> = left
    .to_owned()
    .iter()
    .map(|i| individual_similarity_score(*i, right.clone()))
    .zip(left)
    .map(|(s, t)| s*t)
    .collect();

    similarity_vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part1_first() {
    //     assert_eq!(part1(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 7)
    // }
}
