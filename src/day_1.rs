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

    fn rotate_dial(&mut self, rotation: i32) {
        self.dial += rotation;

        while self.dial < self.dial_min || self.dial > self.dial_max {
            if self.dial < self.dial_min {
                self.dial += self.dial_max+1;
            }
            if self.dial > self.dial_max {
                self.dial -= self.dial_max+1;
            }
        }
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
    
   
    Ok(0)
}

#[test]
fn dial_starts_at_50(){
    let safe = Safe::new();
    assert_eq!(safe.get_dial_position(), 50);
}

#[test]
fn move_dial_forward() {
    let mut safe = Safe::new();
    safe.rotate_dial(14);
    assert_eq!(safe.get_dial_position(), 64);
}

#[test]
fn move_dial_backward() {
    let mut safe = Safe::new();
    safe.rotate_dial(-14);
    assert_eq!(safe.get_dial_position(), 36);
}

#[test]
fn move_dial_forward_past_99() {
    let mut safe = Safe::new();
    safe.rotate_dial(54);
    assert_eq!(safe.get_dial_position(), 4);
}

#[test]
fn move_dial_backward_past_0() {
    let mut safe = Safe::new();
    safe.rotate_dial(-54);
    assert_eq!(safe.get_dial_position(), 96);
}

#[test]
fn rotate_dial_back_to_current_position_forward() {
    let mut safe = Safe::new();
    safe.rotate_dial(100);
    assert_eq!(safe.get_dial_position(), 50);
}

 #[test]
fn rotate_dial_back_to_current_position_backward() {
let mut safe = Safe::new();
    safe.rotate_dial(-100);
    assert_eq!(safe.get_dial_position(), 50);
}

#[test]
fn apply_test_rotations() {
    let rotations = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
    let mut safe = Safe::new();
    let result = apply_rotations(&mut safe, &rotations);
    assert_eq!(result, 3);
}