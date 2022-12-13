use std::str::FromStr;

struct Forest {
    trees: Vec<u32>,
    width: usize,
    height: usize
}

impl Forest {
    fn new() -> Self {
        Self { trees: Vec::new(), width: 0, height: 0 }
    }
}

impl FromStr for Forest {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t:Vec<_> = s.replace("\r\n", "")
                                .chars()
                                .map(|c| c.to_digit(10).unwrap())
                                .collect(); // Row-major vector

        let rl = s.find("\r\n").unwrap();
        let cl = t.len() / rl;

        Ok(Self {
            trees: t,
            width: rl,
            height: cl
        })
    }
}

enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn get_all() -> Vec<Self> {
        vec![Direction::North, Direction::East, Direction::South, Direction:: West]
    }
}

pub fn day8() {
    println!("Day 8");
    let input = include_str!("input/day8.txt");

    if let Ok(forest) = Forest::from_str(input) {
        let mut sum = 0;
    
        for i in 0..forest.trees.len() {
            if is_visible(i, &forest) {
                sum += 1;
            }
        }
    
        println!("Part 1: Trees visible from outside = {}", sum);
    
        let mut best = 0;
    
        for i in 0..forest.trees.len() {
            let score = get_scenic_score(i, &forest);
            if score > best {
                best = score;
            }
        }
    
        println!("Part 1: Best scenic score = {}", best);
    }
    else {
        println!("Error: Failed to parse input");
    }
}

fn get_scenic_score(i: usize, forest: &Forest) -> usize {
    Direction::get_all().into_iter().fold(1, |acc, d| acc * trees_visible_in_direction(d, i, forest))
}

fn is_visible(i: usize, forest: &Forest) -> bool{
    Direction::get_all().into_iter().fold(false, |acc, d| acc || is_visible_in_direction(d, i, forest))
}

fn trees_visible_in_direction(dir: Direction, i: usize, forest: &Forest) -> usize {
    let val = forest.trees.get(i).unwrap();
    if let Some(n) = get_to_border(dir, i, forest) {
        let smaller_trees = n.iter().take_while(|&t| *t < val).count();

        if n.len() > smaller_trees {
            return smaller_trees + 1;
        }
        else {
            return smaller_trees;
        }
    }
    else {
        // no trees in that direction
        return 0;
    }
}

fn is_visible_in_direction(dir: Direction, i: usize, forest: &Forest) -> bool {
    let val = forest.trees.get(i).unwrap();
    if let Some(n) = get_to_border(dir, i, forest) {
        if n.into_iter().max().unwrap() < val {
            return true;
        }
        else {
            return false;
        }
    }
    else {
        return true;
    }
}


fn get_to_border(dir: Direction, i: usize, forest: &Forest) -> Option::<Vec<&u32>> {
    match dir {
        Direction::North => {
            let border_dist = i / forest.height;
            let row_i = i % forest.width;
            if border_dist == 0 {
                return None;
            }
            return Some(forest.trees.iter().skip(row_i)
                                    .step_by(forest.width)
                                    .take(border_dist)
                                    .rev()
                                    .collect());

        }
        Direction::East => {
            let border_dist = forest.width - (i % forest.width) - 1;
            if border_dist == 0 {
                return None;
            }
            return Some(forest.trees.iter().skip(i+1)
                                    .take(border_dist)
                                    .collect());
        }
        Direction::South => {
            let col_i = i / forest.width;
            let border_dist = forest.height - col_i - 1;
            if border_dist == 0 {
                return None;
            }
            return Some(forest.trees.iter().skip(i + forest.width)
                                    .step_by(forest.width)
                                    .take(border_dist)
                                    .collect());
        },
        Direction::West => {
            let border_dist = i % forest.width;
            let col_i = i / forest.width;
            if border_dist == 0 {
                return None;
            }
            return Some(forest.trees.iter().skip(col_i * forest.width)
                                    .take(border_dist)
                                    .rev()
                                    .collect());
        }
    };
}