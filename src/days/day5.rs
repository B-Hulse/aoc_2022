use std::{
    cmp::min, 
    str::FromStr, 
    string::ParseError
};

use regex::Regex;

struct Tower {
    crates: Vec<char>,
}

impl Tower {
    fn new() -> Self {
        Self {
            crates: Vec::new()
        }
    }
}

impl Clone for Tower {
    fn clone(&self) -> Self {
        let mut t = Tower::new();
        t.crates = self.crates.clone();
        t
    }
}

struct Ship {
    towers: Vec<Tower>,
}

impl Ship {
    fn new() -> Self {
        Self {
            towers : Vec::new()
        }
    }

    fn move_crate_serial(&mut self, count : &usize, from: &usize, to: &usize) {
        for _ in 0..*count {
            let from_tower = self.get_or_create_tower_mut(*from-1);
            
            let c = from_tower.crates.pop();

            if c.is_none() {
                return;
            }

            drop(from_tower);

            let to_tower =  self.get_or_create_tower_mut(*to-1);
            
            to_tower.crates.push(c.unwrap());
        }
    }

    fn move_crate(&mut self, count : &usize, from: &usize, to: &usize) {
        let from_tower = self.get_or_create_tower_mut(*from-1);
        let from_len = from_tower.crates.len();
            
        let cnt = min(count, &from_len);
        let skip = from_len - cnt;

        let mut crates= from_tower.crates.drain(skip..).collect::<Vec<_>>();

        drop(from_tower);

        let to_tower =  self.get_or_create_tower_mut(*to-1);
        
        to_tower.crates.append(&mut crates);
    }
    
    fn get_or_create_tower_mut(&mut self, index : usize) -> &mut Tower {
        if index >= self.towers.len() {
            self.towers.resize(index + 1, Tower::new());
        }

        self.towers.get_mut(index).unwrap()
    }
}

impl Clone for Ship {
    fn clone(&self) -> Self {
        let mut new_ship = Ship::new();

        new_ship.towers = self.towers.clone();

        new_ship
    }

}

impl FromStr for Ship {
    type Err = ParseError;

    fn from_str(rows: &str) -> Result<Self, Self::Err> {
        let mut ship = Self::new();

        for row in rows.split("\r\n") {
            for (i, c) in row.chars().skip(1).step_by(4).enumerate() {
                if c == ' ' {
                    continue;
                }

                ship.get_or_create_tower_mut(i).crates.insert(0, c.clone());
            }
        }

        Ok(ship)
    }
}

pub fn day5() {
    let input = include_str!("input/day5.txt");

    let mut ship_1;
    let mut ship_2;

    if let Some(tower_input) = input.split("\r\n\r\n").nth(0) {
        ship_1 = Ship::from_str(tower_input).unwrap();
        ship_2 = Ship::from_str(tower_input).unwrap();
    } else {
        return;
    }

    parse(input).iter().for_each(|(count, from, to)| {
        ship_1.move_crate_serial(&count, &from, &to);
        ship_2.move_crate(&count, &from, &to);
    });

    let answer_1 = ship_1.towers.iter().fold(String::new(), |mut acc, t| {
        if let Some(top) = t.crates.last() {
            acc.push(*top);
        }
        acc
    });

    let answer_2 = ship_2.towers.iter().fold(String::new(), |mut acc, t| {
        if let Some(top) = t.crates.last() {
            acc.push(*top);
        }
        acc
    });

    println!("Day 5");
    println!("Part 1: Top row = {}", answer_1);
    println!("Part 2: Top row = {}", answer_2)
}

fn parse(line: &str) -> Vec<(usize, usize, usize)> {
    // move (\d) from (\d) to (\d)
    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

    let mut instructions = Vec::new();

    for line in re.captures_iter(line) {
        let i1 = line.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let i2 = line.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let i3 = line.get(3).unwrap().as_str().parse::<usize>().unwrap();

        instructions.push((i1, i2, i3));
    }

    instructions
}