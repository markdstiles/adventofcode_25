//https://adventofcode.com/2025/day/4

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 4:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day4.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = reader.lines()
        .map(|line| 
            line.unwrap()
                .chars()
                .collect::<Vec<char>>()
            )
        .collect();
    
    //Process the map looking for forklift locations
    mark_forklift_locations(&mut map);

    //Count forklift locations
    let total = count_forklift_locations(&map);

    Ok(total as i64)
}

fn mark_forklift_locations(map: &mut Vec<Vec<char>>) {

}

fn count_forklift_locations(map: &[Vec<char>]) -> usize {
    map.iter().flatten().filter(|c| **c == 'x').count()
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 4:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day4.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);


    Ok(0 as i64)
}

#[test]
fn day1_proof() {
    let input = [
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    let mut map: Vec<Vec<char>> = input.map(|line| line.chars().collect::<Vec<char>>()).to_vec();

    mark_forklift_locations(&mut map);
    let total = count_forklift_locations(&map);

    assert_eq!(map[0].iter().collect::<String>(), "..xx.xx@x.");
    assert_eq!(map[1].iter().collect::<String>(), "x@@.@.@.@@");
    assert_eq!(map[2].iter().collect::<String>(), "@@@@@.x.@@");
    assert_eq!(map[3].iter().collect::<String>(), "@.@@@@..@.");
    assert_eq!(map[4].iter().collect::<String>(), "x@.@@@@.@x");
    assert_eq!(map[5].iter().collect::<String>(), ".@@@@@@@.@");
    assert_eq!(map[6].iter().collect::<String>(), ".@.@.@.@@@");
    assert_eq!(map[7].iter().collect::<String>(), "x.@@@.@@@@");
    assert_eq!(map[8].iter().collect::<String>(), ".@@@@@@@@.");
    assert_eq!(map[9].iter().collect::<String>(), "x.x.@@@.x.");
    
    assert_eq!(total, 13);
}

#[test]
fn day2_proof() {

}