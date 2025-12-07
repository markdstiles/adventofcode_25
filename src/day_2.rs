//https://adventofcode.com/2025/day/2

use std::{fs::File, io::{BufRead, BufReader, Read}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day2.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut ranges:Vec<(usize, usize)> = Vec::new();

    if let Some(line) = reader.lines().take(1).next() {
        let line = line?;
        ranges = line.split(",")
        .map(|range_str| 
            range_str.split_once("-")
            .map_or((0, 0), |s| (s.0.parse().unwrap(), s.1.parse().unwrap()))).collect();
    }

    let mut total = 0;
    for range in ranges {
        let invalid = find_invalid_ids(range.0, range.1);
        let sum: usize = invalid.iter().sum();
        total += sum;
    }
   
    Ok(total as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day2.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

   
    Ok(0)
}

// lookup table for powers of 10
const POW10: [usize; 20] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000
];

fn get_num_digits(number: usize) -> usize {
    // get bit length
    let b = 64 - number.leading_zeros() as usize;

    // approximage log10 to calculate number of digits
    let mut digits = (b * 1233) >> 12;
    digits += 1;

    if number < POW10[digits - 1] {
        //Account for off-by-one error
        digits -= 1;
    }

    digits
}

fn get_first_half(number: usize) -> usize {
    let digits = get_num_digits(number);

    let k = digits / 2;

    number / POW10[k]

}

fn generate_invalid_id(first_half: usize) -> usize {
    let digits = get_num_digits(first_half);

    first_half * POW10[digits] + first_half
}

/// This function accepts a product id range from, to and searches for invalid ids, returning any as a list
fn find_invalid_ids(from: usize, to: usize) -> Vec<usize> {
    /* An invalid product ID is where the first half of the id matches the second half
    e.g 11, 1010, 987987

    Instead of testing all the numbers in the range to see if they are invalid...
    generate the possible invalid id's less than the 'to' product id.
    */   

    let mut result: Vec<usize> = Vec::new();
    let mut start = from;

    if !get_num_digits(from).is_multiple_of(2) {
        // remove a digit if starting point is not even, 
        // ensures we don't miss any invalid ids e.g 998
        start /= 10;
    }
    let mut number = get_first_half(start);

    loop {
        let invalid_id = generate_invalid_id(number);
        if invalid_id > to {
            break;
        }
        if invalid_id >= from {
            result.push(invalid_id);
        }
        number += 1;
    }

    result
}

#[test]
fn test_get_num_digits() {
    assert_eq!(get_num_digits(1), 1);
    assert_eq!(get_num_digits(99), 2);
    assert_eq!(get_num_digits(123), 3);
    assert_eq!(get_num_digits(5432), 4);
    assert_eq!(get_num_digits(54642), 5);
    assert_eq!(get_num_digits(747467), 6);
    assert_eq!(get_num_digits(8767564), 7);
    assert_eq!(get_num_digits(98743322), 8);
    assert_eq!(get_num_digits(123547899), 9);
    assert_eq!(get_num_digits(7864726423), 10);
    assert_eq!(get_num_digits(34236436666), 11);
    assert_eq!(get_num_digits(674353234644), 12);
    assert_eq!(get_num_digits(2355467678554), 13);
    assert_eq!(get_num_digits(87567632435464), 14);
    assert_eq!(get_num_digits(878767663478783), 15);
    assert_eq!(get_num_digits(2143465465656533), 16);
    assert_eq!(get_num_digits(11124354544443322), 17);
    assert_eq!(get_num_digits(776655424433322211), 18);
    assert_eq!(get_num_digits(1987655789986789875), 19);
    assert_eq!(get_num_digits(18446744073709551614), 20);
}

#[test]
fn test_half_func() {
    assert_eq!(get_first_half(11), 1);
    assert_eq!(get_first_half(123), 12);
    assert_eq!(get_first_half(1234), 12);
    assert_eq!(get_first_half(987654321), 98765);
    assert_eq!(get_first_half(9876543210), 98765);
}

#[test]
fn test_generate_func() {
    assert_eq!(generate_invalid_id(1), 11);
    assert_eq!(generate_invalid_id(2), 22);
    assert_eq!(generate_invalid_id(32), 3232);
    assert_eq!(generate_invalid_id(1234), 12341234);
}

#[test]
fn range_with_no_repeated() {
    let from = 1;
    let to = 9;
    let result = find_invalid_ids(from, to);
    assert_eq!(result.len(), 0);
}

#[test]
fn single_repeated() {
    let from = 1;
    let to = 12;
    let result = find_invalid_ids(from, to);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], 11);
}

#[test]
fn single_repeated_2() {
    let from = 998;
    let to = 1020;
    let result = find_invalid_ids(from, to);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], 1010);
}

#[test]
fn two_repeated() {
    let from = 11;
    let to = 22;
    let result = find_invalid_ids(from, to);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], 11);
    assert_eq!(result[1], 22);
}

#[test]
fn no_repeated() {
    let from = 1698522;
    let to = 1698528;
    let result = find_invalid_ids(from, to);
    assert_eq!(result.len(), 0);
}

#[test]
fn day1_proof() {
    let ranges = [
        (11,22),
        (95,115),
        (998,1012),
        (1188511880,1188511890),
        (222220,222224),
        (1698522,1698528),
        (446443,446449),
        (38593856,38593862),
        (565653,565659),
        (824824821,824824827),
        (2121212118,2121212124)
    ];
    let mut total: usize = 0;
    for range in ranges {
        let invalid = find_invalid_ids(range.0, range.1);
        let sum: usize = invalid.iter().sum();
        total += sum;
    }
    assert_eq!(total, 1227775554);
}