use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

use crate::utils::input;

fn generate_sample() -> String {
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()
}

fn generate_sample3() -> String {
    let test = generate_sample();
    part1_input(test.as_str())
}

fn calculate_length(start: usize, end: usize) -> usize {
    if end >= start {
        end - start
    } else {
        panic!("End <= start: End: {}, start; {}", end, start)
    }
}

fn remove_redundant_starts_and_ends(starts: Vec<usize>, ends: Vec<usize>, input_length: usize) -> (Vec<usize>, Vec<usize>) {
    let next_closest_ends: Vec<usize> = starts
    .iter()
    .map(|x| *ends.iter().find(|&i| x<i).unwrap_or(&input_length))
    .collect::<Vec<usize>>();
    
    // Debug statements
    // println!("Starts: {:?}", starts);
    // println!("Next closest ends: {:?}", next_closest_ends);
    
    let mut deduped_ends = next_closest_ends.to_owned();
    deduped_ends.dedup();
    
    // Debug statements
    // println!("Deduped ends: {:?}", deduped_ends);
    
    let mut deduped_starts: Vec<usize> = vec![starts[0]];
    deduped_starts.extend(deduped_ends
    .iter()
    .map(|x| *starts.iter().find(|&i| x<=i).unwrap_or(&0))
    .collect::<Vec<usize>>());
    deduped_starts.pop();

    (deduped_starts.to_owned(), deduped_ends.to_owned())
}

fn multiplier_from_input(input: &str, part: usize) -> Vec<(usize, usize)> {

    let filtered: String = if part == 2 {
        let filter_start = Regex::new(r"don't\(\)").unwrap();
        let filter_end = Regex::new(r"do\(\)").unwrap();

        let intermediate_filter_starts: Vec<usize> = filter_start.find_iter(input).map(|m| m.start()).collect();
        let intermediate_filter_ends: Vec<usize> = filter_end.find_iter(input).map(|m| m.end()).collect();

        // Debug statements
        // println!("Regex starts: {:?}", intermediate_filter_starts);
        // println!("Regex ends: {:?}", intermediate_filter_ends);

        let (filter_starts, filter_ends) = remove_redundant_starts_and_ends(intermediate_filter_starts, intermediate_filter_ends, input.len());

        if (filter_starts.len().abs_diff(filter_ends.len()) > 1) || (filter_starts.len() < filter_ends.len()) {
            panic!("Filter lengths incorrect! Starts: {}, ends: {}, starts: {:?}, ends: {:?}", filter_starts.len(), filter_ends.len(), filter_starts, filter_ends)
        };
        let mut starts: Vec<usize> = vec![0usize];
        starts.extend(filter_ends.to_owned());
        let mut ends: Vec<usize> = filter_starts.to_owned();
        if filter_starts.len() == filter_ends.len() {
            ends.push(input.len());
        };

        let lengths: Vec<usize> = starts.iter().zip(ends).map(|(s, e)| calculate_length(*s,e)).collect();

        let snippets: Vec<String> = starts.iter().zip(lengths).map(|(s, l)| input.chars().skip(*s).take(l).collect::<String>()).collect();

        let filtered_string = snippets.join("");

        filtered_string
    } else if part != 1 {
        panic!("Incorrect part supplied: {}", part)
    } else {
        input.to_string()
    };
    
    let instruction_regex = Regex::new(r"(?:mul\()([0-9]+,[0-9]+)(?:\))").unwrap(); // structured with groups

    let mut full_matches: Vec<&str> = vec![];
    let mut matches: Vec<&str> = vec![];

    for (orig, [cap]) in instruction_regex.captures_iter(filtered.as_str()).map(|c| c.extract()) {
        full_matches.push(orig);
        matches.push(cap);
    }

    let result: Vec<(usize, usize)> = matches
    .into_iter()
    .enumerate()
    .map(
        |(n,i)| input::usize_from_enum_strln(n, i, ",", 2).into_iter().collect_tuple::<(usize, usize)>().unwrap()
    )
    .collect();

    result
}

#[aoc_generator(day3)]
fn part1_input(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1, mine)]
fn part1(input: &String) -> usize {
    let test_input: String = generate_sample3();
    let test_sample: Vec<(usize, usize)> = multiplier_from_input(test_input.as_str(), 1);
    let test_results: Vec<usize> = test_sample.iter().map(|i| i.0 * i.1).collect();
    println!("Test result: {}", test_results.into_iter().sum::<usize>());

    let results: Vec<usize> = multiplier_from_input(input.as_str(), 1).iter().map(|i| i.0 * i.1).collect();
    results.into_iter().sum::<usize>()
}

#[aoc(day3, part2, mine)]
fn part2(input: &String) -> usize {
    let test_input: String = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
    let test_sample: Vec<(usize, usize)> = multiplier_from_input(test_input.as_str(), 2);
    let test_results: Vec<usize> = test_sample.iter().map(|i| i.0 * i.1).collect();
    println!("Test result: {}", test_results.into_iter().sum::<usize>());

    let results: Vec<usize> = multiplier_from_input(input.as_str(), 2).iter().map(|i| i.0 * i.1).collect();
    results.into_iter().sum::<usize>()
}