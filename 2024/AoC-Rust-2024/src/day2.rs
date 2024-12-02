use aoc_runner_derive::{aoc, aoc_generator};
// use itertools::Itertools;
// use std::collections::HashSet;

fn check_safety(level: &Vec<usize>) -> bool {
    let diffs: Vec<usize> = level.windows(2).map(|w| w[0].abs_diff(w[1])).collect();
    // Debug
    // println!("Diffs of {:?}: {:?}", level, diffs);
    // println!("Sorted: {}", level.is_sorted());
    // println!("Filtered: {:?}", diffs.clone().into_iter().filter(|&x| x<1 || x>3).collect::<Vec<usize>>());
    // println!("Filtered length: {}", diffs.clone().into_iter().filter(|&x| x<1 || x>3).collect::<Vec<usize>>().len());

    let mut rev: Vec<usize> = level.to_owned();
    rev.reverse();

    (level.is_sorted() || rev.is_sorted()) && diffs.into_iter().filter(|&x| x<1 || x>3).collect::<Vec<usize>>().len() == 0
}

fn check_dampened_safety(level: &Vec<usize>) -> bool {
    if check_safety(level) { true }
    else {
        let dampened: Vec<Vec<usize>> = (0..level.len())
        .map(
            |i| {
                let mut t = level.to_owned().to_vec();
                t.remove(i);
                t
            }
        )
        .collect();

        for v in dampened {
            if check_safety(&v) { return true }
        }

        false
    }
}

fn usize_from_unbounded_enum_strln(n: usize, line: &str, sep: &str) -> Vec<usize> {
    let result: Vec<usize> = line
        .split(sep)
        .map(|i| {
            i.parse::<usize>().expect(
                format!(
                    "Could not parse {} from input line {}: {} into usize!",
                    i,
                    n,
                    line,
                ).as_str()
            )
        })
        .collect();

    // if result.len() != length {
    //     panic! {
    //         "Length error! Input line {}: {}, expected length: {}, found: {}",
    //         n,
    //         line,
    //         length,
    //         result.len(),
    //     }
    // }

    result
}

fn generate_sample() -> String {
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9".to_string()
}

fn generate_sample2() -> Vec<Vec<usize>> {
    let test_str = generate_sample();
    part1_input(test_str.as_str())
}

#[aoc_generator(day2)]
fn part1_input(input: &str) -> Vec<Vec<usize>> {
    let result: Vec<Vec<usize>> = input.lines().enumerate().map(|(n,v)| usize_from_unbounded_enum_strln(n, v, " ")).collect();

    // let lengths: HashSet<usize> = result.iter().map(|v| v.len()).collect();

    // if lengths != 1 {
    //     panic!("Uneven vector lengths!")
    // }

    result
}

#[aoc(day2, part1, mine)]
fn part1(input: &[Vec<usize>]) -> usize {
    let test_safety: Vec<bool> = generate_sample2().iter().map(|v| check_safety(v)).collect();
    println!("Test safeties: {:?}", test_safety);
    println!("Test safety count: {}", test_safety.into_iter().filter(|b| *b).count());

    let safety: Vec<bool> = input.iter().map(|v| check_safety(v)).collect();
    safety.into_iter().filter(|b| *b).count()
    // 0
}

#[aoc(day2, part2, mine)]
fn part2(input: &[Vec<usize>]) -> usize {
    let test_safety: Vec<bool> = generate_sample2().iter().map(|v| check_dampened_safety(v)).collect();
    println!("Test safeties: {:?}", test_safety);
    println!("Test safety count: {}", test_safety.into_iter().filter(|b| *b).count());

    let safety: Vec<bool> = input.iter().map(|v| check_dampened_safety(v)).collect();
    safety.into_iter().filter(|b| *b).count()
    // 0
}