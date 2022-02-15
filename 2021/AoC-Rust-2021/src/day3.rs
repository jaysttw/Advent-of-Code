use aoc_runner_derive::{aoc, aoc_generator};
// use itertools::Itertools;
// use byteorder::{BigEndian, ReadBytesExt};
use std::str;
use std::error::Error;

#[aoc_generator(day3)]
fn day3_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_owned()).collect() // need to map to bits
}

#[aoc(day3, part1)]
fn day3_part1(input: &[Vec<u8>]) -> i32 {
    // with heavy borrowing from https://github.com/rbtcollins/aoc-2021/blob/main/src/day3.rs
    let mut current_count: Vec<i8> = vec![0; input[0].len()];
    for line in input {
        for bit in 0..line.len() {
            let val: i8 = (line[bit]).try_into().unwrap();
            current_count[bit] = current_count[bit] + (val * 2) - 1; // adds 1 if bit is 1, 0 otherwise.
        }
    }

    println!("current_count: {:?}", current_count);

    let mut epsilon = 0;
    let mut gamma = 0;
    for bit in current_count {
        gamma <<= 1;
        epsilon <<= 1;
        match bit {
            i if i < 0 => epsilon += 1,
            i if i > 0 => gamma += 1,
            _ => panic!("Error in matching bit {:?}.", bit)
        }
    }

    gamma * epsilon
}

fn generate_readings(input: &[Vec<u8>], oxygen: bool, next: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut inner_variable: i8 = 0;
    for line in input {
        let val: i8 = line[next].try_into().unwrap();
        inner_variable = inner_variable + (val * 2) - 1;
    }
    let current_bit: u8 = match oxygen {
        true => match inner_variable {
            i if i >= 0 => 1,
            i if i < 0 => 0,
            _ => panic!("Oxygen counter received: {:?}", inner_variable)
        },
        false => match inner_variable {
            i if i >= 0 => 0,
            i if i < 0 => 1,
            _ => panic!("CO2 counter received: {:?}", inner_variable)
        }
    };
    let working_input: Vec<Vec<u8>> = input.iter().filter(|l| l[next] == current_bit).cloned().collect();
    if working_input.len() == 1 {
        Ok(vec![current_bit])
    } else {
        let other = generate_readings(&working_input, oxygen, next+1).unwrap();
        Ok(vec![current_bit].iter().copied().chain(other.iter().copied()).collect::<Vec<u8>>())
    }
}

#[aoc(day3, part2)]
fn day3_part2(input: &[Vec<u8>]) -> u8 {
    /* Logic:
        1. Iterate through the input, and generate the count for the first bit.
        2. Remove the offending entries according to the logic.
            * O2: Keep most common bit, 1 as tie-breaker.
            * CO2: Keep least common bit, 0 as tie-breaker.
        3. Iterate through the input again, and generate the count again.
        4. Continue until only 1 input remains OR only 1 digit remains.
            * The latter condition implies the former, so only the former needs to be implemented.
    */
    let oxygen_vec: Vec<u8> = generate_readings(input, true, 0).unwrap();
    let carbon_dioxide_vec: Vec<u8> = generate_readings(input, false, 0).unwrap();

    // let oxygen: u32 = u32::from_be_bytes(oxygen_vec);
    // let carbon_dioxide: u32 = u32::from_be_bytes(carbon_dioxide_vec);

    let mut oxygen = 0;
    for bit in 0..oxygen_vec.len() {
        oxygen <<= 1;
        oxygen += oxygen_vec[bit];
    }

    let mut carbon_dioxide = 0;
    for bit in 0..carbon_dioxide_vec.len() {
        carbon_dioxide <<= 1;
        carbon_dioxide += carbon_dioxide_vec[bit];
    }
    
    oxygen * carbon_dioxide
}