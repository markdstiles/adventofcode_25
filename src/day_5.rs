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

    fn count(&self) -> usize {
        (self.to - self.from) + 1
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
 
    let mut ranges: Vec<FreshRange> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.contains("-") {
            ranges.push(FreshRange::from_string(line));
        }
    }

    ranges.sort_by(|a, b| a.from.cmp(&b.from));

    //Consolidate duplicate or overlapping ranges
    let mut unique_ranges: Vec<FreshRange> = vec![];
    
    for range in ranges {
        if let Some(u_range) = unique_ranges.iter_mut().filter(|u| range.from >= u.from && range.from <= u.to).last() {
            //Ensure the 'to' value is greater than current unique range (don't want to make it smaller)
            if u_range.to < range.to {
                u_range.to = range.to;
            }
        } else {
            unique_ranges.push(range);
        }
    }

    let total: usize = unique_ranges.iter().map(|r| r.count()).sum();

    Ok(total as i64)
}

#[test]
fn part1_proof() {
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

#[test]
fn part2_proof() {
    let mut ranges = [
        FreshRange::new(3, 5),
        FreshRange::new(10, 14),
        FreshRange::new(16, 20),
        FreshRange::new(12, 18),
    ];

    ranges.sort_by(|a, b| a.from.cmp(&b.from));

    //Consolidate duplicate or overlapping ranges
    let mut unique_ranges: Vec<FreshRange> = vec![];
    
    for range in ranges {
        if let Some(u_range) = unique_ranges.iter_mut().filter(|u| range.from >= u.from && range.from <= u.to).last() {
            u_range.to = range.to;
        } else {
            unique_ranges.push(range);
        }
    }

    let total: usize = unique_ranges.iter().map(|r| r.count()).sum();

    assert_eq!(total, 14);
    assert_eq!(unique_ranges.len(), 2);
    assert_eq!(unique_ranges[0].from, 3);
    assert_eq!(unique_ranges[0].to, 5);
    assert_eq!(unique_ranges[1].from, 10);
    assert_eq!(unique_ranges[1].to, 20);
}

#[test]
fn day2_inner_range() {
    let mut ranges = [
        FreshRange::new(3, 5),
        FreshRange::new(10, 14),
        FreshRange::new(16, 20),
        FreshRange::new(12, 18),
        FreshRange::new(17, 19),
    ];

    ranges.sort_by(|a, b| a.from.cmp(&b.from));

    //Consolidate duplicate or overlapping ranges
    let mut unique_ranges: Vec<FreshRange> = vec![];
    
    for range in ranges {
        if let Some(u_range) = unique_ranges.iter_mut().filter(|u| range.from >= u.from && range.from <= u.to).last() {
            //Ensure the 'to' value is greater than current unique range (don't want to make it smaller)
            if u_range.to < range.to {
                u_range.to = range.to;
            }
        } else {
            unique_ranges.push(range);
        }
    }

    let total: usize = unique_ranges.iter().map(|r| r.count()).sum();

    assert_eq!(total, 14);
    assert_eq!(unique_ranges.len(), 2);
    assert_eq!(unique_ranges[0].from, 3);
    assert_eq!(unique_ranges[0].to, 5);
    assert_eq!(unique_ranges[1].from, 10);
    assert_eq!(unique_ranges[1].to, 20);
}