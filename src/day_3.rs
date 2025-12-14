//https://adventofcode.com/2025/day/3

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day3.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut bank_width = 0;
    let mut bank_count = 0;
    let mut batts = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let x: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap_or_default() as u8).collect();
        batts.extend_from_slice(x.as_slice());
        bank_width = line.len();
        bank_count += 1;
    }

    let mut batt_array = BatteryArray::new(bank_width, bank_count);
    batt_array.batteries.copy_from_slice(batts.as_slice());

    let total_joltage = batt_array.get_total_joltage();

    Ok(total_joltage as i64)
}

struct BatteryArray {
    batteries: Vec<u8>,
    bank_width: usize,
    bank_count: usize,
}

impl BatteryArray {
    fn new(bank_width: usize, bank_count: usize) -> Self {
        BatteryArray { batteries: vec![0; bank_width * bank_count] , bank_width, bank_count }
    }

    fn get_bank_joltage(&self, bank_idx: usize) -> usize {
        if bank_idx >= self.bank_count {
            0
        } else {
            let mut joltage: usize = 0;
            let mut max = 0;
            let mut max_idx = 0;

            let offset = bank_idx * self.bank_width;
            //First battery
            for i in 0..self.bank_width-1 {
                let curr = self.batteries[offset + i];
                if curr > max {
                    max = curr;
                    max_idx = i;
                }
            }
            joltage = self.batteries[offset + max_idx] as usize * 10;
            
            //Second battery
            max = 0;
            let start = max_idx+1;
            for j in start..self.bank_width {
                let curr = self.batteries[offset + j];
                if curr > max {
                    max = curr;
                    max_idx = j;
                }
            }
            joltage += self.batteries[offset + max_idx] as usize;

            joltage
        }
    }

    fn get_total_joltage(&self) -> usize {
        let mut total = 0;

        for bank in 0..self.bank_count {
            total += self.get_bank_joltage(bank)
        }

        total
    }
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day3.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    Ok(0)
}

#[test]
fn day1_proof() {
    let batts : [u8; 60] = [
        9,8,7,6,5,4,3,2,1,1,1,1,1,1,1,
        8,1,1,1,1,1,1,1,1,1,1,1,1,1,9,
        2,3,4,2,3,4,2,3,4,2,3,4,2,7,8,
        8,1,8,1,8,1,9,1,1,1,1,2,1,1,1,
    ];
    let mut batt_array = BatteryArray::new(15, 4);
    for (i, joltage) in batts.iter().enumerate() {
        batt_array.batteries[i] = *joltage;
    }

    assert_eq!(batt_array.get_bank_joltage(0), 98);
    assert_eq!(batt_array.get_bank_joltage(1), 89);
    assert_eq!(batt_array.get_bank_joltage(2), 78);
    assert_eq!(batt_array.get_bank_joltage(3), 92);

    assert_eq!(batt_array.get_total_joltage(), 357);
}