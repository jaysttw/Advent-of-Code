use aoc_runner_derive::{aoc, aoc_generator};
// use itertools::Itertools;

enum Play {
    Rock,
    Paper,
    Scissors,
}

struct Round(Play, Play);

#[aoc_generator(day1)]
fn part1_input(input: &str) ->  {
    // should panic if unexpected input found
    let vec_input: Vec<Round> = input.lines().map(|l| {
        l.trim().to_string()
        if l.len() != 2 { panic!("Unexpected input!") };

        let other: Play = match l[0] {
            "A" => Some(Play::Rock),
            "B" => Some(Play::Paper),
            "C" => Some(Play::Scissors),
            _ => todo!()
        }
        let own: Play match l[1] {
            "X" => Some(Play::Rock),
            "Y" => Some(Play::Paper),
            "Z" => Some(Play::Scissors),
            _ => todo!()
        }
        
        Round(other, own)
    }).collect();

    

    vec_input
}

#[aoc(day2, part1, mine)]
fn part1(input: &[Vec<Round>]) -> i32 {
    // let result = input.iter().map(|v| v.iter().sum()).collect::<Vec<i32>>();
    // sum_of_n_highest(&result, 1)
    // result.into_iter().max().unwrap()
}

#[aoc(day2, part2, mine)]
fn part2(input: &[Vec<Round>]) -> i32 {
    // let result = input.iter().map(|v| v.iter().sum()).collect::<Vec<i32>>();
    // sum_of_n_highest(&result, 3)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // #[test]
    // fn part1_first() {
    //     assert_eq!(part1(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 7)
    // }
}