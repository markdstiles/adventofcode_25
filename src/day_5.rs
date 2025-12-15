//https://adventofcode.com/2025/day/5

use std::{fs::File, io::{BufRead, BufReader}};

struct FreshRange {
    from: usize,
    to: usize,
}

impl FreshRange {
    fn new(from: usize, to: usize) -> Self {
        FreshRange { from, to }
    }

    fn from_string(range_str: String) -> Self {
        let range: Vec<usize> = range_str.split("-").map(|s| s.parse().unwrap()).collect();
        FreshRange { from: range[0], to: range[1] }
    }
}

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 5:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day5.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut ranges: Vec<FreshRange> = vec![];
    let mut ingredients: Vec<usize> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.contains("-") {
            ranges.push(FreshRange::from_string(line));
        } else if !line.is_empty() {
            let ingredient: usize = line.parse()?;
            ingredients.push(ingredient);
        }
    }

    let fresh: Vec<usize> = ingredients.iter()
        .filter(|&&i| ranges.iter()
            .any(|r| i >= r.from && i <= r.to))
        .copied()
        .collect();

    Ok(fresh.len() as i64)
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

#[test]
fn day1_proof() {
    let mut ranges = [
        FreshRange::new(3, 5),
        FreshRange::new(10, 14),
        FreshRange::new(16, 20),
        FreshRange::new(12, 18),
    ];

    let ingredients: [usize; 6] = [
        1,
        5,
        8,
        11,
        17,
        32,
    ];

    ranges.sort_by(|a, b| a.from.cmp(&b.from));

    assert_eq!(ranges[2].from, 12);
    
    let fresh: Vec<usize> = ingredients.iter()
        .filter(|&&i| ranges.iter()
            .any(|r| i >= r.from && i <= r.to))
        .copied()
        .collect();

    assert_eq!(fresh.len(), 3);
    assert_eq!(fresh[0], 5);
    assert_eq!(fresh[1], 11);
    assert_eq!(fresh[2], 17);
}