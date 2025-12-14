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
    let cols = map[0].len();
    let rows = map.len();

    for row in 0..rows {
        for col in 0..cols {
            //Only consider cell if it is a roll of paper
            //Detect surrounding rolls '@' or marked rolls 'x'
            if map[row][col] == '@' {
                let mut rolls = 0;

                //Keep within left and right edges
                let col_from = if col == 0 { col } else { col-1 };
                let col_to = if col == cols-1 { col } else { col+1 };
                
                if row != 0 {
                    //Previous row - unless we're at the top edge
                    rolls = map[row-1][col_from..=col_to].iter().filter(|c| **c == '@' || **c == 'x').count();
                }

                //Left & right
                rolls += map[row][col_from..=col_to].iter().filter(|c| **c == '@' || **c == 'x').count();
                rolls -= 1; //Ignore current cell
                
                if row != rows-1 {
                    //Next row - unless we're at the bottom  edge
                    rolls += map[row+1][col_from..=col_to].iter().filter(|c| **c == '@' || **c == 'x').count();
                }

                if rolls < 4 {
                    //Accessible - mark cell
                    map[row][col] = 'x'
                }
            }
        }
    }
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
    let _reader = BufReader::new(file);


    Ok(0 as i64)
}

#[test]
fn day1_proof() {
    let mut map: Vec<Vec<char>> = vec![
        "..@@.@@@@.".chars().collect::<Vec<char>>(),
        "@@@.@.@.@@".chars().collect::<Vec<char>>(),
        "@@@@@.@.@@".chars().collect::<Vec<char>>(),
        "@.@@@@..@.".chars().collect::<Vec<char>>(),
        "@@.@@@@.@@".chars().collect::<Vec<char>>(),
        ".@@@@@@@.@".chars().collect::<Vec<char>>(),
        ".@.@.@.@@@".chars().collect::<Vec<char>>(),
        "@.@@@.@@@@".chars().collect::<Vec<char>>(),
        ".@@@@@@@@.".chars().collect::<Vec<char>>(),
        "@.@.@@@.@.".chars().collect::<Vec<char>>(),
    ];
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