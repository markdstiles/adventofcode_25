//https://adventofcode.com/2025/day/7

use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

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

fn traverse_tree(x: isize, y: usize, visited: &mut HashMap<(isize, usize), usize>, map: &Vec<Vec<char>>, map_height: usize, map_width: isize) -> usize {
    let next_row = y + 1;

    if let Some(result) = visited.get(&(x, y)) {
        //If visited, no need to explore further
        return *result
    }

    if next_row == map_height {
        //Record that we've visited the bottom
        visited.entry((x, y)).or_insert(1);
        1
    } else if next_row < map_height && map[next_row][x as usize] != '^' {
        traverse_tree(x, next_row, visited, map, map_height, map_width)
    }
    else {
        let mut result = 0;
        //Split
        if x-1 >= 0 {
            result += traverse_tree(x-1, next_row, visited, map, map_height, map_width);
        }
        if x+1 < map_width {
            result += traverse_tree(x+1, next_row, visited, map, map_height, map_width);
        }
        //Record when both branches have been visited - no further need to explore these branches
        //This reduces repeated exploration of branches
        visited.entry((x, y)).or_insert(result);
        result
    }
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 7:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day7.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let map: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();

    let start_pos_y = 0;
    let start_pos_x =  if let Some(start) = map[0].iter().position(|c| *c == 'S') {
        start
    } else {
        0
    } as isize;

    let map_height = map.len();
    let map_width = map[0].len() as isize;
    let mut visited: HashMap<(isize, usize), usize> = HashMap::new();
    let timeline_count = traverse_tree(start_pos_x, start_pos_y, &mut visited, &map, map_height, map_width);

    Ok(timeline_count as i64)
}

#[test]
fn day1_proof() {
    let input = [
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

#[test]
fn day2_proof() {
    //We'll skip every other line (all .'s)
    let input = [
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
    
    let map: Vec<Vec<char>> = input.map(|line| line.chars().collect()).to_vec();
    let start_pos_y = 0;
    let start_pos_x =  if let Some(start) = map[0].iter().position(|c| *c == 'S') {
        start
    } else {
        0
    } as isize;

    let map_height = map.len();
    let map_width = map[0].len() as isize;
    let mut visited: HashMap<(isize, usize), usize> = HashMap::new();
    let timeline_count = traverse_tree(start_pos_x, start_pos_y, &mut visited, &map, map_height, map_width);

    assert_eq!(timeline_count, 40);
}