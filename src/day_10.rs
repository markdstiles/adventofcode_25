//https://adventofcode.com/2025/day/10

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 10:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day10.txt");

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
fn day1_proof() {

}

#[test]
fn day2_proof() {
    
}