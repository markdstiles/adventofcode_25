//https://adventofcode.com/2025/day/8

use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 8:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day8.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut input: Vec<Position> = vec![];

    for line in reader.lines() {
        let line = line?;
        let value: Vec<u64> = line.split(",").map(|s| s.parse().unwrap_or_default()).collect();
        input.push(Position::new(value[0], value[1], value[2]));
    }

    let mut pairs: Vec<JunctionPair> = vec![];

    for i in 0..input.len() {
        let first = input[i];

        for j in i+1..input.len() {
            let second = input[j];
            pairs.push(JunctionPair::new(first, second));
        }
    }
    //Order by its absolute value (this is not the distance as we're not applying square root for performance)
    pairs.sort_by(|a, b| a.absolute.cmp(&b.absolute));

    let mut circuits = generate_circuits(&pairs, 1000);
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    let result = circuits[0].len() * circuits[1].len() * circuits[2].len();

    Ok(result as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 8:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day8.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);


    Ok(0 as i64)
}

fn sqr(n: u64) -> u64 {
    n * n
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Position {
    x: u64,
    y: u64,
    z: u64,
}

impl Position {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Position { x, y, z }
    }

    fn abs(&self, other: Position) -> u64 {
        sqr(self.x.abs_diff(other.x))
        + sqr(self.y.abs_diff(other.y))
        + sqr(self.z.abs_diff(other.z))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct JunctionPair {
    first: Position,
    second: Position,
    absolute: u64,
}

impl JunctionPair {
    fn new(first: Position, second: Position) -> Self {
        let absolute = first.abs(second);
        JunctionPair { first, second, absolute }
    }
}

fn generate_circuits(pairs: &Vec<JunctionPair>, cycles: usize) -> Vec<Vec<Position>> {
    let mut circuits: Vec<Vec<Position>> = vec![];
    let mut position_circuit_index: HashMap<Position, usize> = HashMap::new();

    for (i, pair) in pairs.iter().enumerate() {
        if i == cycles {
            break;
        }
        //Lookup circuit for first or second position
        if let Some(&circuit1) = position_circuit_index.get(&pair.first) {
            if let Some(&circuit2) = position_circuit_index.get(&pair.second) {
                //If they aren't in the same circuit already...
                if circuit1 != circuit2 {
                    for pos in &circuits[circuit2] {
                        //Update index for circuit2 positions
                        if let Some(val) = position_circuit_index.get_mut(&pos) {
                            *val = circuit1;
                        }
                    }

                    //Move circuit2 to circuit1 to join them
                    let mut from: Vec<Position> = circuits[circuit2].iter().copied().collect();
                    circuits[circuit1].append(&mut from);
                    circuits[circuit2].clear();
                }
            } else {
                //Add first position to circuit2
                circuits[circuit1].push(pair.second);
                //Add to index
                position_circuit_index.insert(pair.second, circuit1);
            }
        } else {
            if let Some(&circuit2)  = position_circuit_index.get(&pair.second) {
                //Add first position to circuit2
                circuits[circuit2].push(pair.first);
                //Add to index
                position_circuit_index.insert(pair.first, circuit2);
            } else {
                //Add first and second to new circuit
                circuits.push(vec![pair.first, pair.second]);
                //Add to index
                let idx = circuits.len()-1;
                position_circuit_index.insert(pair.first, idx);
                position_circuit_index.insert(pair.second, idx);
            }
        }
    }

    circuits
}

#[test]
fn part1_proof() {
    let input: [(u64, u64, u64); 20] = [
        (162,817,812),
        (57,618,57),
        (906,360,560),
        (592,479,940),
        (352,342,300),
        (466,668,158),
        (542,29,236),
        (431,825,988),
        (739,650,466),
        (52,470,668),
        (216,146,977),
        (819,987,18),
        (117,168,530),
        (805,96,715),
        (346,949,466),
        (970,615,88),
        (941,993,340),
        (862,61,35),
        (984,92,344),
        (425,690,689),
    ];

    let mut pairs: Vec<JunctionPair> = vec![];

    for i in 0..input.len() {
        let first = Position::new(input[i].0, input[i].1, input[i].2);

        for j in i+1..input.len() {
            let second = Position::new(input[j].0, input[j].1, input[j].2);
            pairs.push(JunctionPair::new(first, second));
        }
    }

    //Order by its absolute value (this is not the distance as we're not applying square root for performance)
    pairs.sort_by(|a, b| a.absolute.cmp(&b.absolute));

    assert_eq!(pairs[0], JunctionPair::new(Position::new(162, 817, 812), Position::new(425, 690, 689)));

    let mut circuits = generate_circuits(&pairs, 10);
    
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    assert_eq!(circuits[0].len(), 5);
    assert_eq!(circuits[1].len(), 4);
    assert_eq!(circuits[2].len(), 2);
}

#[test]
fn part2_proof() {

}