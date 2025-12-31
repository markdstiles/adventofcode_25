//https://adventofcode.com/2025/day/9

use std::{char::EscapeDefault, fs::File, io::{BufRead, BufReader, BufWriter, Write}, usize};

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

    let mut rects: Vec<Rect> = generate_rects(&points);
    //Sort largest to smallest
    rects.sort_by(|a, b| b.area.cmp(&a.area));

    //Generate a list of edges with their direction
    let edges = find_edges(&points);
    let corners = find_corners(&edges);

    //Find the largest rectangle, but reject any where:
    // 1. Intersects any other edges (interior of rectangle must be 'clean')
    //  a. intersection where edge ends within rectangle
    //  b. intersection where edge starts and ends outside rectangle but cuts across it
    // 2. Rectangle, corner to corner is on the exterior of the perimeter

    //Starting with the largest, test each rectangle until we find a valid one
    let mut largest_rectangle: Option<Rect> = None;

    for rect in rects {
        let mut is_valid = true;

        for edge in &edges {
            if rect.intersects(&edge) {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            if let Some(bottom_corner) = corners.iter().find(|c| c.vertex == rect.bottom_corner) {
                if let Some(top_corner) = corners.iter().find(|c| c.vertex == rect.top_corner) {
                    // Compare corners, does the rectangle span the interior, enclosed by the perimeter?
                    is_valid = bottom_corner.enclosed(top_corner);
                }
            }
        }

        if is_valid {
            largest_rectangle = Some(rect);
            break;
        }
    }

    let mut result = 0;

    if let Some(rect) = largest_rectangle {
        result = rect.area;
    };

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
    bottom_corner: Point,
    top_corner: Point,
    width: usize,
    height: usize,
    area: usize,
}

impl Rect {
    fn new(corner1: Point, corner2: Point) -> Self {
        let width = corner1.x.abs_diff(corner2.x) + 1;
        let height = corner1.y.abs_diff(corner2.y) + 1;
        let area = width * height;

        if corner1.y <= corner2.y {
            Rect { bottom_corner: corner1, top_corner: corner2, width, height, area }
        } else {
            Rect { bottom_corner: corner2, top_corner: corner1, width, height, area }
        }
    }

    fn inside(&self, p: Point) -> bool {
        p.y > self.bottom_corner.y
        && p.y < self.top_corner.y
        && ((self.bottom_corner.x <= self.top_corner.x //Bottom corner is left
            && p.x > self.bottom_corner.x
            && p.x < self.top_corner.x)
            ||
            (self.bottom_corner.x > self.top_corner.x //Bottom corner is right
            && p.x > self.top_corner.x
            && p.x < self.bottom_corner.x
            ))
    }

    fn intersects(&self, e: &Edge) -> bool {
        //Either ends of the edge are inside the rectangle
        /*
        ##########
        #        #
        #     -----
        ##########
         */
        self.inside(e.from) || self.inside(e.to)
        //The edge spans the rectangle from outside
        || match e.direction {
            EdgeDirection::Down => {
                //Vertical intersection test
                /*
                    |
                ####|####
                #   |   #
                ####|####
                    |
                */
                e.from.y >= self.top_corner.y
                && e.to.y <= self.bottom_corner.y
                && self.inside(Point::new(e.from.x, self.bottom_corner.y + 1))
            },
            EdgeDirection::Up => {
                e.from.y <= self.top_corner.y
                && e.to.y >= self.bottom_corner.y
                && self.inside(Point::new(e.from.x, self.bottom_corner.y + 1))
            },
            EdgeDirection::Left => {
                //Horizontal intersection test
                /*
                 #########
                 #       #
                -----------
                 #       #
                 #########
                */
                ((self.bottom_corner.x <= self.top_corner.x //Bottom corner is left
                && e.from.x >= self.bottom_corner.x
                && e.to.x <= self.top_corner.x)
                ||
                (self.bottom_corner.x > self.top_corner.x //Bottom corner is right
                && e.from.x <= self.top_corner.x
                && e.to.x >= self.bottom_corner.x
                ))
                && e.from.y < self.top_corner.y
                && e.from.y > self.bottom_corner.y
            },
            EdgeDirection::Right => {
                ((self.bottom_corner.x <= self.top_corner.x //Bottom corner is left
                && e.to.x >= self.bottom_corner.x
                && e.from.x <= self.top_corner.x)
                ||
                (self.bottom_corner.x > self.top_corner.x //Bottom corner is right
                && e.to.x >= self.top_corner.x
                && e.from.x <= self.bottom_corner.x
                ))
                && e.from.y < self.top_corner.y
                && e.from.y > self.bottom_corner.y
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EdgeDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    from: Point,
    to: Point,
    direction: EdgeDirection,
}

impl Edge {
    fn new(from: Point, to: Point) -> Self {
        let direction = if from.x == to.x 
        && to.y < from.y {
            EdgeDirection::Down
        } else if from.x < to.x 
        && from.y == to.y {
            EdgeDirection::Right
        } else if from.x == to.x
        && from.y < to.y {
            EdgeDirection::Up
        } else {
            EdgeDirection::Left
        };

        Edge { from, to, direction }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CornerDirection {
    //Clockwise
    RightDown,
    DownLeft,
    LeftUp,
    UpRight,
    //Anti-clockwise
    LeftDown,
    DownRight,
    RightUp,
    UpLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Corner {
    vertex: Point,
    from: Edge,
    to: Edge,
    direction: CornerDirection,
}

impl Corner {
    fn new(from: Edge, to: Edge) -> Self {
        assert_eq!(from.to, to.from);

        let direction = match (from.direction, to.direction) {
            //Clockwise
            (EdgeDirection::Right, EdgeDirection::Down) => CornerDirection::RightDown,
            (EdgeDirection::Down, EdgeDirection::Left) => CornerDirection::DownLeft,
            (EdgeDirection::Left, EdgeDirection::Up) => CornerDirection::LeftUp,
            (EdgeDirection::Up, EdgeDirection::Right) => CornerDirection::UpRight,
            //Anti-clockwise
            (EdgeDirection::Left, EdgeDirection::Down) => CornerDirection::LeftDown,
            (EdgeDirection::Down, EdgeDirection::Right) => CornerDirection::DownRight,
            (EdgeDirection::Right, EdgeDirection::Up) => CornerDirection::RightUp,
            (EdgeDirection::Up, EdgeDirection::Left) => CornerDirection::UpLeft,
            _ => CornerDirection::RightDown,
        };

        Corner { vertex: from.to, from, to, direction }
    }

    fn enclosed(&self, other: &Corner) -> bool {
        /* In this example A, B, & C are all visible to X & Y the space in between is the interior
        and the perimeter direction is clockwise, A is occluded from Z

        #############
        #           X##Y
        #  A####B      #
        #  #    #      #
        ####    C######Z

         */ 
        match self.direction {
            CornerDirection::RightDown | CornerDirection::DownRight => other.direction != CornerDirection::RightDown || other.direction != CornerDirection::DownRight,
            CornerDirection::DownLeft | CornerDirection::LeftDown => other.direction != CornerDirection::DownLeft || other.direction != CornerDirection::LeftDown,
            CornerDirection::LeftUp | CornerDirection::UpLeft => other.direction != CornerDirection::LeftUp || other.direction != CornerDirection::UpLeft,
            CornerDirection::UpRight | CornerDirection::RightUp => other.direction != CornerDirection::UpRight || other.direction != CornerDirection::RightUp,
        }
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

fn find_edges(points: &Vec<Point>) -> Vec<Edge> {
    let mut edges = vec![];

    for i in 0..points.len()-1 {
        edges.push(Edge::new(points[i], points[i+1]));
    }

    edges.push(Edge::new(points[points.len()-1], points[0]));

    edges
}

fn find_corners(edges: &Vec<Edge>) -> Vec<Corner> {
    let mut corners = vec![];

    for i in 0..edges.len()-1 {
        corners.push(Corner::new(edges[i], edges[i+1]));
    }

    corners.push(Corner::new(edges[edges.len()-1], edges[0]));

    corners
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
    //Generate rectangle combinations as per part 1
    let mut rects: Vec<Rect> = generate_rects(&points);
    //Sort largest to smallest
    rects.sort_by(|a, b| b.area.cmp(&a.area));

    //Generate a list of edges with their direction
    let edges = find_edges(&points);
    let corners = find_corners(&edges);

    //Find the largest rectangle, but reject any where:
    // 1. Intersects any other edges (interior of rectangle must be 'clean')
    //  a. intersection where edge ends within rectangle
    //  b. intersection where edge starts and ends outside rectangle but cuts across it
    // 2. Rectangle, corner to corner is on the exterior of the perimeter

    //Starting with the largest, test each rectangle until we find a valid one
    let mut largest_rectangle: Option<Rect> = None;

    for rect in rects {
        let mut is_valid = true;

        for edge in &edges {
            if rect.intersects(&edge) {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            if let Some(bottom_corner) = corners.iter().find(|c| c.vertex == rect.bottom_corner) {
                if let Some(top_corner) = corners.iter().find(|c| c.vertex == rect.top_corner) {
                    // Compare corners, does the rectangle span the interior, enclosed by the perimeter?
                    is_valid = bottom_corner.enclosed(top_corner);
                }
            }
        }

        if is_valid {
            largest_rectangle = Some(rect);
            break;
        }
    }

    assert!(largest_rectangle.is_some());
    if let Some(rect) = largest_rectangle {
        assert_eq!(rect.area, 24);
    };
}

#[test]
fn intersection_tests() {
    let rect = Rect::new(Point::new(2,3), Point::new(9,7));

    //Left edge
    assert!(rect.intersects(&Edge::new(Point::new(9, 5), Point::new(2, 5))));
    //Right edge
    assert!(rect.intersects(&Edge::new(Point::new(2, 5), Point::new(9, 5))));
    //Up edge
    assert!(rect.intersects(&Edge::new(Point::new(4, 2), Point::new(4, 7))));
    //Down edge
    assert!(rect.intersects(&Edge::new(Point::new(4, 7), Point::new(4, 2))));
}