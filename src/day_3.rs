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

    let mut batt_array = BatteryArray::new(bank_width, bank_count, 2);
    batt_array.batteries.copy_from_slice(batts.as_slice());

    let total_joltage = batt_array.get_total_joltage();

    Ok(total_joltage as i64)
}

struct BatteryArray {
    batteries: Vec<u8>,
    bank_width: usize,
    bank_count: usize,
    batteries_required: usize,
}

impl BatteryArray {
    fn new(bank_width: usize, bank_count: usize, batteries_required: usize) -> Self {
        BatteryArray { batteries: vec![0; bank_width * bank_count] , bank_width, bank_count, batteries_required }
    }

    fn get_bank_joltage(&self, bank_idx: usize) -> usize {
        // Can make this more efficient, depending on the number of batteries
        // to find this will search the remainder of the bank repeatedly, but 
        // the range will gradually get smaller
        if bank_idx >= self.bank_count {
            0
        } else {
            let mut battery_jolts: Vec<usize> = vec![0; self.batteries_required];
            let mut selected_batteries: Vec<usize> = vec![0; self.batteries_required];

            let bank_offset = bank_idx * self.bank_width;

            for battery in 0..self.batteries_required {
                // If its the first battery we search for the highest 'joltage' from the start of the bank
                // otherwise start at the next battery from the previous highest joltage
                let start = if battery == 0 { 0 } else { selected_batteries[battery-1] + 1 };

                // We don't need to search all the way to the end of the bank, this will depend on how many
                // batteries we're looking for in total
                let end = self.bank_width - (self.batteries_required - (battery + 1));

                for idx in start..end {
                    if battery_jolts[battery] < self.batteries[bank_offset + idx] as usize {
                        battery_jolts[battery] = self.batteries[bank_offset + idx] as usize;
                        selected_batteries[battery] = idx;

                        if battery_jolts[battery] == 9 {
                            // 9 is the highest we can find so stop looking
                            break;
                        }
                    }
                }
            }

            let mut joltage = 0;
            
            //Calculate joltage
            for jolts in battery_jolts {
                joltage = joltage * 10 + jolts;
            }

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

    let mut batt_array = BatteryArray::new(bank_width, bank_count, 12);
    batt_array.batteries.copy_from_slice(batts.as_slice());

    let total_joltage = batt_array.get_total_joltage();

    Ok(total_joltage as i64)
}

#[test]
fn day1_proof() {
    let batts : [u8; 60] = [
        9,8,7,6,5,4,3,2,1,1,1,1,1,1,1,
        8,1,1,1,1,1,1,1,1,1,1,1,1,1,9,
        2,3,4,2,3,4,2,3,4,2,3,4,2,7,8,
        8,1,8,1,8,1,9,1,1,1,1,2,1,1,1,
    ];
    let mut batt_array = BatteryArray::new(15, 4, 2);
    for (i, joltage) in batts.iter().enumerate() {
        batt_array.batteries[i] = *joltage;
    }

    assert_eq!(batt_array.get_bank_joltage(0), 98);
    assert_eq!(batt_array.get_bank_joltage(1), 89);
    assert_eq!(batt_array.get_bank_joltage(2), 78);
    assert_eq!(batt_array.get_bank_joltage(3), 92);

    assert_eq!(batt_array.get_total_joltage(), 357);
}

#[test]
fn day2_proof() {
    let batts : [u8; 60] = [
        9,8,7,6,5,4,3,2,1,1,1,1,1,1,1,
        8,1,1,1,1,1,1,1,1,1,1,1,1,1,9,
        2,3,4,2,3,4,2,3,4,2,3,4,2,7,8,
        8,1,8,1,8,1,9,1,1,1,1,2,1,1,1,
    ];
    let mut batt_array = BatteryArray::new(15, 4, 12);
    for (i, joltage) in batts.iter().enumerate() {
        batt_array.batteries[i] = *joltage;
    }

    assert_eq!(batt_array.get_bank_joltage(0), 987654321111);
    assert_eq!(batt_array.get_bank_joltage(1), 811111111119);
    assert_eq!(batt_array.get_bank_joltage(2), 434234234278);
    assert_eq!(batt_array.get_bank_joltage(3), 888911112111);

    assert_eq!(batt_array.get_total_joltage(), 3121910778619);
}