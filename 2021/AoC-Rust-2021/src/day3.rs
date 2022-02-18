use aoc_runner_derive::{aoc, aoc_generator};
// use itertools::Itertools;
// use byteorder::{BigEndian, ReadBytesExt};
use std::str;
use std::error::Error;

#[aoc_generator(day3)]
fn day3_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_owned()).collect() // need to map to bytes
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
    let mut current_count: i8 = 0;
    println!("Assigned inner variable for next: {:?}, input length: {:}", next, input.len()); // debug
    for line in input {
        let val: i8 = line[next].try_into().unwrap();
        current_count = current_count + (val * 2) - 1;
    }
    println!("Current count ({:}): {:}", next, current_count);
    let current_bit: u8 = match oxygen {
        true => match current_count {
            i if i >= 0 => 1,
            i if i < 0 => 0,
            _ => panic!("Oxygen counter received: {:?}", current_count)
        },
        false => match current_count {
            i if i >= 0 => 0,
            i if i < 0 => 1,
            _ => panic!("CO2 counter received: {:?}", current_count)
        }
    };
    // println!("{:}: {:}, {:}", next, current_count, current_bit);
    let test_bit: u8 = current_bit.to_string().as_bytes().to_owned()[0];
    let working_input: Vec<Vec<u8>> = input.iter().filter(|l| l[next] == test_bit).cloned().collect();
    // println!("New working input ({:}): {:}", next, working_input.len());
    if working_input.len() <= 1 {
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

    println!("Oxygen vector: {:?}, length: {:}", oxygen_vec, oxygen_vec.len());

    let mut oxygen = 0;
    for bit in 0..oxygen_vec.len() {
        oxygen *= 2;
        oxygen += oxygen_vec[bit];
    }

    println!("CO2 vector:    {:?}, length: {:}", carbon_dioxide_vec, carbon_dioxide_vec.len());

    let mut carbon_dioxide = 0;
    for bit in 0..carbon_dioxide_vec.len() {
        carbon_dioxide *= 2;
        carbon_dioxide += carbon_dioxide_vec[bit];
    }
    
    oxygen * carbon_dioxide
}