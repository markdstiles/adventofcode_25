//https://adventofcode.com/2025/day/6

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 6:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day6.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut ops: Vec<char> = vec![];
    let mut numbers: Vec<Vec<usize>> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.chars().nth(0).unwrap_or_else(|| '0').is_ascii_punctuation() {
            ops = line.replace(" ", "").chars().collect();
        } else {
            numbers.push(line.trim().split(" ").filter(|x| !x.is_empty()).map(|s| s.parse().unwrap()).collect());
        }
    }

    let mut total: usize = 0;

    for (i, op) in ops.iter().enumerate() {
        total += match op {
            '*' => numbers.iter().fold(1_usize, |acc, n| acc * n[i]),
            '+' => numbers.iter().fold(0_usize, |acc, n| acc + n[i]),
            _ => 0,
        };
    }

    Ok(total as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 6:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day6.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
 
    Ok(0 as i64)
}

#[test]
fn day1_proof() {
    let input = [
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
    ];
    let ops: Vec<char> = "*   +   *   +  ".replace(" ", "").chars().collect();

    let mut numbers: Vec<Vec<usize>> = vec![];
    for line in input {
        numbers.push(line.trim().split(" ").filter(|x| !x.is_empty()).map(|s| s.parse().unwrap()).collect());
    }

    let mut total: usize = 0;

    for (i, op) in ops.iter().enumerate() {
        total += match op {
            '*' => numbers.iter().fold(1_usize, |acc, n| acc * n[i]),
            '+' => numbers.iter().fold(0_usize, |acc, n| acc + n[i]),
            _ => 0,
        };
    }

    assert_eq!(total, 4277556);
    assert_eq!(numbers.len(), 3);
    assert_eq!(numbers[0].len(), 4);
    assert_eq!(ops.len(), 4);
    assert_eq!(ops[0], '*');
}