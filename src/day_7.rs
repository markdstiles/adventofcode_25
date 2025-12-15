//https://adventofcode.com/2025/day/7

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 7:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day7.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();

    if let Some(start) = map[0].iter().position(|c| *c == 'S') {
        map[0][start] = '|';
    }

    let mut split_count: usize = 0;
    let map_height = map.len();
    let map_width = map[0].len();

    for y in 1..map_height {
        for x in 0..map_width {
            if map[y-1][x] == '|' {
                if map[y][x] == '^' {
                    split_count += 1;
                    if x != 0 {
                        map[y][x-1] = '|';
                    }
                    if x != map_width - 1 {
                        map[y][x+1] = '|';
                    }
                } else {
                    map[y][x] = '|';
                }
            }
        }
    }

    Ok(split_count as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 7:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day7.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    Ok(0 as i64)
}

#[test]
fn day1_proof() {
    let mut input = [
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
   ];

   let mut map: Vec<Vec<char>> = input.map(|line| line.chars().collect()).to_vec();
   if let Some(start) = map[0].iter().position(|c| *c == 'S') {
    map[0][start] = '|';
   }

   let mut split_count: usize = 0;
   let map_height = map.len();
   let map_width = map[0].len();

   for y in 1..map_height {
    for x in 0..map_width {
        if map[y-1][x] == '|' {
            if map[y][x] == '^' {
                split_count += 1;
                if x != 0 {
                    map[y][x-1] = '|';
                }
                if x != map_width - 1 {
                    map[y][x+1] = '|';
                }
            } else {
                map[y][x] = '|';
            }
        }
    }
   }

   assert_eq!(split_count, 21);
}