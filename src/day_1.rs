//https://adventofcode.com/2025/day/1

use std::{fs::File, io::{BufRead, BufReader}};

struct Safe {
    dial: i32,
    dial_min: i32,
    dial_max: i32,
}

impl Safe {
    fn new() -> Safe {
        Safe {
            dial: 50,
            dial_min: 0,
            dial_max: 99,
        }
    }

    fn rotate_dial(&mut self, rotation: i32) -> i32 {
        if rotation == 0 {
            return 0
        }

        let dial_started_at_0 = self.dial == 0;
        self.dial += rotation;

        if self.dial == 0 {
            return 1
        }

        let mut zero_count = 0; // counter for the number of times we pass 0

        // for positive values we can simply use division and the remainder to determine the dial position and times we passed 0
        if self.dial > self.dial_max {
            zero_count = self.dial / (self.dial_max+1);
            self.dial %= self.dial_max+1;
        }

        // for negative values we'll simulate turning the dial
        if self.dial < self.dial_min {
            zero_count = if dial_started_at_0 { 0 } else { 1 }; // we have already passed 0 once unless the dial started on 0

            while self.dial < self.dial_min {
                self.dial += self.dial_max+1;

                if self.dial <= self.dial_min {
                    // if the dial is still 0 or negative we hit 0
                    zero_count += 1;
                }
            }
        }

        zero_count
    }

    fn get_dial_position(&self) -> i32 {
        self.dial
    }
}

fn apply_rotations(safe: &mut Safe, rotations: &[i32]) -> i32 {
    let mut result = 0;

    for rotation in rotations {
        safe.rotate_dial(*rotation);

        if safe.get_dial_position() == 0 {
            result += 1;
        }
    }

    result
}

fn apply_rotations_part2(safe: &mut Safe, rotations: &[i32]) -> i32 {
    let mut result = 0;

    for rotation in rotations {
        result += safe.rotate_dial(*rotation);
    }

    result
}

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day1.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let mut rotations: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.replace("L", "-");
        let line = line.replace("R", "+");
        rotations.push(line.parse()?);
    }

    let mut safe = Safe::new();
    let zeros = apply_rotations(&mut safe, &rotations) as i64;
   
    Ok(zeros)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day1.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let mut rotations: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.replace("L", "-");
        let line = line.replace("R", "+");
        rotations.push(line.parse()?);
    }

    let mut safe = Safe::new();
    let zeros = apply_rotations_part2(&mut safe, &rotations) as i64;
   
    Ok(zeros)
}

#[test]
fn dial_starts_at_50(){
    let safe = Safe::new();
    assert_eq!(safe.get_dial_position(), 50);
}

#[test]
fn move_dial_forward() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(14);
    assert_eq!(safe.get_dial_position(), 64);
    assert_eq!(zero_count, 0);
}

#[test]
fn move_dial_backward() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(-14);
    assert_eq!(safe.get_dial_position(), 36);
    assert_eq!(zero_count, 0);
}

#[test]
fn move_dial_forward_past_99() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(54);
    assert_eq!(safe.get_dial_position(), 4);
    assert_eq!(zero_count, 1);
}

#[test]
fn move_dial_backward_past_0() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(-54);
    assert_eq!(safe.get_dial_position(), 96);
    assert_eq!(zero_count, 1);
}

#[test]
fn rotate_dial_back_to_current_position_forward() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(100);
    assert_eq!(safe.get_dial_position(), 50);
    assert_eq!(zero_count, 1);
}

#[test]
fn rotate_dial_back_to_current_position_backward() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(-100);
    assert_eq!(safe.get_dial_position(), 50);
    assert_eq!(zero_count, 1);
}

#[test]
fn rotate_dial_back_to_current_position_forward_1000() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(1000);
    assert_eq!(safe.get_dial_position(), 50);
    assert_eq!(zero_count, 10);
}

#[test]
fn rotate_dial_to_0_forward_150() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(150);
    assert_eq!(safe.get_dial_position(), 0);
    assert_eq!(zero_count, 2);
}

#[test]
fn rotate_dial_to_0_backward_150() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(-150);
    assert_eq!(safe.get_dial_position(), 0);
    assert_eq!(zero_count, 2);
}

#[test]
fn rotate_dial_to_0_backward_250() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(-250);
    assert_eq!(safe.get_dial_position(), 0);
    assert_eq!(zero_count, 3);
}

#[test]
fn rotate_dial_back_to_current_position_backward_1050() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(-1050);
    assert_eq!(safe.get_dial_position(), 0);
    assert_eq!(zero_count, 11);
}

#[test]
fn rotate_dial_to_zero_then_to_0_again_backward() {
    let mut safe = Safe::new();
    let zero_count = safe.rotate_dial(-50);
    assert_eq!(safe.get_dial_position(), 0);
    assert_eq!(zero_count, 1, "move to 0 from 50");
    let zero_count = safe.rotate_dial(-100);
    assert_eq!(safe.get_dial_position(), 0);
    assert_eq!(zero_count, 1, "move to 0 from 0 (left)");
}

#[test]
fn apply_test_rotations() {
    let rotations = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
    let mut safe = Safe::new();
    let result = apply_rotations(&mut safe, &rotations);
    assert_eq!(result, 3);
}

#[test]
fn apply_test_rotations_part2() {
    let rotations = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
    let mut safe = Safe::new();
    let result = apply_rotations_part2(&mut safe, &rotations);
    assert_eq!(result, 6);
}