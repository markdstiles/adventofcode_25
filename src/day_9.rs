//https://adventofcode.com/2025/day/9

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 9:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day9.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut points: Vec<Point> = vec![];
    for line in reader.lines() {
        let line = line?;

        let (x, y) = if let Some((x_str, y_str)) = line.split_once(",") {
            (x_str.parse()?, y_str.parse()?)
        } else {
            (0, 0)
        };
        
        points.push(Point::new(x, y));
    }

    let mut rects = generate_rects(&points);
    rects.sort_by(|a, b| b.area.cmp(&a.area));

    let result = rects[0].area;

    Ok(result as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 8:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day8.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let result = 0;

    Ok(result as i64)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Rect {
    corner1: Point,
    corner2: Point,
    width: usize,
    height: usize,
    area: usize,
}

impl Rect {
    fn new(corner1: Point, corner2: Point) -> Self {
        let width = corner1.x.abs_diff(corner2.x) + 1;
        let height = corner1.y.abs_diff(corner2.y) + 1;
        let area = width * height;
        Rect { corner1, corner2, width, height, area }
    }
}

fn generate_rects(points: &Vec<Point>) -> Vec<Rect> {
    let mut rects: Vec<Rect> = vec![];

    for i in 0..points.len() {
        let corner1 = points[i];
        for j in i+1..points.len() {
            let corner2 = points[j];
            rects.push(Rect::new(corner1, corner2));
        }
    }

    rects
}

#[test]
fn part1_proof() {
    let input: [(usize, usize); 8] = [
        (7,1),
        (11,1),
        (11,7),
        (9,7),
        (9,5),
        (2,5),
        (2,3),
        (7,3),
    ];

    let points: Vec<Point> = input.iter().map(|(x, y)| Point::new(*x, *y)).collect();

    let mut rects = generate_rects(&points);
    rects.sort_by(|a, b| b.area.cmp(&a.area));
    
    assert_ne!(rects.len(), 0);
    assert_eq!(rects[0].area, 50);
}

#[test]
fn part2_proof() {
    let input: [(usize, usize); 8] = [
        (7,1),
        (11,1),
        (11,7),
        (9,7),
        (9,5),
        (2,5),
        (2,3),
        (7,3),
    ];

    let points: Vec<Point> = input.iter().map(|(x, y)| Point::new(*x, *y)).collect();
}