use aoc_runner_derive::{aoc, aoc_generator};
// use itertools::Itertools;

#[aoc_generator(day1)]
fn part1_input(input: &str) -> Vec<Vec<i32>> {
    // should panic if unexpected input found
    let vec_input = input.lines().map(|l| {
        l.trim().to_string()
    }).collect::<Vec<String>>();

    let output = vec_input.into_iter().fold(Vec::new(), |mut acc, x| {
        if x.is_empty() || acc.is_empty() {
            acc.push(Vec::new());
        };
        if !x.is_empty() { acc.last_mut().unwrap().push(x.parse::<i32>().unwrap()); };
        acc
    });

    output
}

#[aoc(day1, part1, mine)]
fn part1(input: &[Vec<i32>]) -> i32 {
    let result = input.iter().map(|v| v.iter().sum()).collect::<Vec<i32>>();
    sum_of_n_highest(&result, 1)
    // result.into_iter().max().unwrap()
}

#[aoc(day1, part2, mine)]
fn part2(input: &[Vec<i32>]) -> i32 {
    let result = input.iter().map(|v| v.iter().sum()).collect::<Vec<i32>>();
    sum_of_n_highest(&result, 3)
}

fn sum_of_n_highest(input: &[i32], n: usize) -> i32 {
    let mut newinput = input.to_vec();
    newinput.sort();
    newinput.reverse(); // beware, might not be good for all reversed sorts (https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust)
    newinput[0..n].iter().sum()
}

// #[aoc(day1, part2)]
// fn part2(input: &[i32]) -> usize {
//     input.iter().tuple_windows::<(_,_,_,_)>().filter(|w| w.0 < w.3).count()
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    // #[test]
    // fn part1_first() {
    //     assert_eq!(part1(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 7)
    // }
}