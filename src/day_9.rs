//https://adventofcode.com/2025/day/9

use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 9:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day9.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let result = 0;

    Ok(result as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 8:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day8.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let result = 0;

    Ok(result as i64)
}

#[test]
fn part1_proof() {
    let input: [(usize, usize); 8] = [
        (7,1),
        (11,1),
        (11,7),
        (9,7),
        (9,5),
        (2,5),
        (2,3),
        (7,3),
    ];
}

#[test]
fn part2_proof() {
    let input: [(usize, usize); 8] = [
        (7,1),
        (11,1),
        (11,7),
        (9,7),
        (9,5),
        (2,5),
        (2,3),
        (7,3),
    ];
}