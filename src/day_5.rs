//https://adventofcode.com/2025/day/5

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 5:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day5.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    Ok(0 as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 5:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day5.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    Ok(0 as i64)
}