//https://adventofcode.com/2025/day/10

use core::fmt;
use std::{collections::{HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 10:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day10.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut input = Input::new();
    for line in reader.lines() {
        let line = line?;
        input.parse_line(line.to_string());
    }

    //List of successfully initialised machines
    //Aim: find the fewest toggle patterns to match the desired light config.
    //The order we apply the toggle patterns doesn't matter.
    //XOR is commutative, we only need to test combinations, not permutations.
    //Create a test machine for each toggle pattern. We want to test all patterns
    //simulataneously so we can find the fewest number of them as quickly as possible.

    //List of successfully initialised machines
    let mut machines: Vec<MachineState> = vec![];

    for (machine_idx, target) in input.targets.iter().enumerate() {
        let mut machines_to_test: VecDeque<MachineState> = VecDeque::new();
        let toggles = &input.toggles[machine_idx];
        let toggle_set = &input.toggle_sets[machine_idx];

        //Setup a test machine with one of each toggle app
        for toggle in toggles {
            let mut test_machine = MachineState::new(*target);
            //Apply first toggle
            test_machine.apply_toggle(*toggle);
            machines_to_test.push_back(test_machine);
        }

        //Search for an initialised machine
        let machine = 'search: loop {
            if let Some(machine) = machines_to_test.pop_front() {
                if machine.is_initialised() {
                    //Are we initialised from applying the first toggle in the setup phase?
                    break 'search machine
                }

                for toggle in toggles {
                    let mut test_machine = machine.copy();

                    if test_machine.test_toggle(*toggle, toggle_set) {
                        test_machine.apply_toggle(*toggle);

                        if test_machine.is_initialised() {
                            break 'search test_machine
                        }

                        machines_to_test.push_back(test_machine);
                    }
                }
            }
        };

        machines.push(machine);
    }

    let result = machines.iter().fold(0, |acc, x| acc + x.toggles_applied.len());

    Ok(result as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 10:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day10.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let result = 0;

    Ok(result as i64)
}

struct Input {
    targets: Vec<u16>,
    toggles: Vec<Vec<u16>>,
    toggle_sets: Vec<HashSet<u16>>,
    joltages: Vec<Vec<u8>>,
}

impl Input {
    fn new() -> Self {
        Input { targets: vec![], toggles: vec![], toggle_sets: vec![], joltages: vec![] }
    }

    fn parse_line(&mut self, line: String) {
        let mut line_toggles: Vec<u16> = vec![];
        for seg in line.split(" ") {
            match seg.chars().nth(0) {
                Some('[') => self.targets.push(light_config_to_binary(&seg[1..seg.len()-1].chars().collect::<Vec<char>>())),
                Some('(') => line_toggles.push(index_to_binary(&(seg[1..seg.len()-1]).split(",").filter_map(|s| s.parse().ok()).collect::<Vec<u8>>())),
                Some('{') => self.joltages.push(seg[1..seg.len()-1].split(",").filter_map(|s| s.parse().ok()).collect()),
                _ => {},
            }
        }
        self.toggle_sets.push(line_toggles.iter().copied().collect());
        self.toggles.push(line_toggles);
    }
}

fn index_to_binary(indices: &[u8]) -> u16 {    
    indices.iter().fold(0, |acc, i| acc | 1 << *i)
}

fn light_config_to_binary(config: &[char]) -> u16 {
    index_to_binary(&config.iter().enumerate().filter_map(|(i, c)| match c { '#' => Some(i as u8), _ => None }).collect::<Vec<u8>>())
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MachineState {
    light_state: u16,
    target: u16,
    toggles_applied: Vec<u16>,
}

impl MachineState {
    fn new(target_state: u16) -> Self {
        MachineState { light_state: 0, target: target_state, toggles_applied: vec![] }
    }

    fn apply_toggle(&mut self, toggle: u16) {
        self.light_state ^= toggle;
        self.toggles_applied.push(toggle);
    }

    fn test_toggle(&self, toggle:u16, toggle_set: &HashSet<u16>) -> bool {
        let new_state = self.light_state ^ toggle;
        !(
            new_state == self.light_state           //Must change the state
            || new_state == 0                       //Must not return to the initial state
            || toggle_set.contains(&new_state)      //Must not create a state that is one of the toggles
            || self.toggles_applied.last().is_some_and(|t| *t == toggle)    //Toggle must not be one of the last 2 applied
            || self.toggles_applied.iter().rev().nth(1).is_some_and(|t| *t == toggle)
        )     

    }

    fn is_initialised(&self) -> bool {
        self.light_state == self.target
    }

    fn reset(&mut self) {
        self.light_state = 0;
        self.toggles_applied.clear();
    }

    fn copy(&self) -> MachineState {
        MachineState { 
            light_state: self.light_state, 
            target: self.target, 
            toggles_applied: self.toggles_applied.iter().copied().collect() 
        }
    }
}

impl fmt::Debug for MachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MachineState[ls:{:b} target:{:b} applied:[", self.light_state, self.target)?;
        for applied in self.toggles_applied.iter() {
            write!(f, "{:b},", *applied)?;
        }
        write!(f, "]]")
    }
}

#[test]
fn day1_proof() {
    let input_lines = [
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        "[.###..##.] (3,5,6,8) (0,1,3,4,5,7) (0,3,4,5,6,8) (3,8) (1,2,3,5,6,7) (0,1,2,3,4,5,8) (1,2,5,6,7) (1,2,3,4,5,6) (0,2,3,5) (7,8) {35,40,50,101,38,87,67,25,57}",
        "[###.#...#] (1,2,3,5,7) (0,1,2,5,7,8) (1,2,3,5,6,8) (0,2,3,5,6,7) (0,1,2,3,5,8) (0) (0,4,5) (0,1,4,8) (0,1,2,3,4,5,8) {93,57,60,41,35,78,27,44,51}",
    ];

    let mut input = Input::new();
    for line in input_lines {
        input.parse_line(line.to_string());
    }

    //Aim: find the fewest toggle patterns to match the desired light config.
    //The order we apply the toggle patterns doesn't matter.
    //XOR is commutative, we only need to test combinations, not permutations.
    //Create a test machine for each toggle pattern. We want to test all patterns
    //simulataneously so we can find the fewest number of them as quickly as possible.

    //List of successfully initialised machines
    let mut machines: Vec<MachineState> = vec![];

    for (machine_idx, target) in input.targets.iter().enumerate() {
        let mut machines_to_test: VecDeque<MachineState> = VecDeque::new();
        let toggles = &input.toggles[machine_idx];
        let toggle_set = &input.toggle_sets[machine_idx];

        //Setup a test machine with one of each toggle app
        for toggle in toggles {
            let mut test_machine = MachineState::new(*target);
            //Apply first toggle
            test_machine.apply_toggle(*toggle);
            machines_to_test.push_back(test_machine);
        }

        //Search for an initialised machine
        let machine = 'search: loop {
            if let Some(machine) = machines_to_test.pop_front() {
                if machine.is_initialised() {
                    //Are we initialised from applying the first toggle in the setup phase?
                    break 'search machine
                }

                for toggle in toggles {
                    let mut test_machine = machine.copy();

                    if test_machine.test_toggle(*toggle, toggle_set) {
                        test_machine.apply_toggle(*toggle);

                        if test_machine.is_initialised() {
                            break 'search test_machine
                        }

                        machines_to_test.push_back(test_machine);
                    }
                }
            }
        };

        machines.push(machine);
    }

    assert_eq!(machines.len(), 5);
    assert_eq!(machines[0].toggles_applied.len(), 2);
    assert_eq!(machines[1].toggles_applied.len(), 3);
    assert_eq!(machines[2].toggles_applied.len(), 2);
    assert_eq!(machines[3].toggles_applied.len(), 4);
    assert_eq!(machines[4].toggles_applied.len(), 8);
    assert_eq!(machines.iter().fold(0, |acc, x| acc + x.toggles_applied.len()), 19);
}

#[test]
fn day2_proof() {
}

#[test]
fn index_to_binary_tests() {
    let indices = [0];
    assert_eq!(index_to_binary(&indices), 1);

    let indices = [1];
    assert_eq!(index_to_binary(&indices), 2);

    let indices = [2];
    assert_eq!(index_to_binary(&indices), 4);

    let indices = [3];
    assert_eq!(index_to_binary(&indices), 8);

    let indices = [4];
    assert_eq!(index_to_binary(&indices), 16);

    let indices = [0,1];
    assert_eq!(index_to_binary(&indices), 3);

    let indices = [2,3];
    assert_eq!(index_to_binary(&indices), 12);
}

#[test]
fn light_config_to_binary_tests() {
    let config = ['#','.','.'];
    assert_eq!(light_config_to_binary(&config), 1);

    let config = ['#','#','.'];
    assert_eq!(light_config_to_binary(&config), 3);

    let config = ['.', '#','#','.'];
    assert_eq!(light_config_to_binary(&config), 6);

    let config = ['.', '#','#','#'];
    assert_eq!(light_config_to_binary(&config), 14);
}

#[test]
fn test_input_parsing() {
    let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    let mut input = Input::new();
    input.parse_line(line.to_string());

    assert_eq!(input.targets.len(), 1);
    assert_eq!(input.targets[0], 6);
    assert_eq!(input.toggles.len(), 1);
    assert_eq!(input.toggles[0].len(), 6);
    assert_eq!(input.toggles[0][0], 8);
    assert_eq!(input.toggles[0][1], 10);
    assert_eq!(input.toggles[0][2], 4);
    assert_eq!(input.joltages.len(), 1);
    assert_eq!(input.joltages[0].len(), 4);
    assert_eq!(input.joltages[0][0], 3);
    assert_eq!(input.joltages[0][1], 5);
    assert_eq!(input.joltages[0][2], 4);
    assert_eq!(input.joltages[0][3], 7);
}

#[test]
fn day1_manual_test() {
    let input_lines = [
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
    ];

    let mut input = Input::new();
    for line in input_lines {
        input.parse_line(line.to_string());
    }

    //Manually solve example puzzle to check MachineState is working

    let mut machines: Vec<MachineState> = vec![];
    let mut machine1 = MachineState::new(input.targets[0]);
    machine1.apply_toggle(input.toggles[0][1]);
    machine1.apply_toggle(input.toggles[0][3]);
    assert!(machine1.is_initialised());
    machines.push(machine1);

    let mut machine2 = MachineState::new(input.targets[1]);
    machine2.apply_toggle(input.toggles[1][4]);
    machine2.apply_toggle(input.toggles[1][3]);
    machine2.apply_toggle(input.toggles[1][2]);
    assert!(machine2.is_initialised());
    machines.push(machine2);

    let mut machine3 = MachineState::new(input.targets[2]);
    machine3.apply_toggle(input.toggles[2][1]);
    machine3.apply_toggle(input.toggles[2][2]);
    assert!(machine3.is_initialised());
    machines.push(machine3);

    assert_eq!(machines.len(), 3);
    assert_eq!(machines[0].toggles_applied.len(), 2);
    assert_eq!(machines[1].toggles_applied.len(), 3);
    assert_eq!(machines[2].toggles_applied.len(), 2);
    assert_eq!(machines.iter().fold(0, |acc, x| acc + x.toggles_applied.len()), 7);
}