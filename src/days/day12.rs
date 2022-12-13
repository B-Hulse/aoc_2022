use std::{str::FromStr, collections::{HashMap, VecDeque}};

#[derive(Clone, Copy, PartialEq)]
struct Vec2 {
    x: usize,
    y: usize
}

impl Vec2 {
    fn new(x: usize, y:usize) -> Self {
        Self { x: x, y: y }
    }

    fn step(&self, dir: Dir) -> Option<Self> {
        let mut out = self.clone();
        match dir {
            Dir::North => {
                if out.y == 0 {
                    return None;
                }
                else {
                    out.y -= 1;
                    return Some(out);
                }
            }
            Dir::South => {
                out.y += 1;
                return Some(out);
            }
            Dir::West => {
                if out.x == 0 {
                    return None;
                }
                else {
                    out.x -= 1;
                    return Some(out);
                }
            }
            Dir::East => {
                out.x += 1;
                return Some(out);
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West
}

#[derive(Clone)]
struct TopoMap {
    map: Vec<i32>,
    dim: Vec2,
    start: usize,
    dest: usize
}

impl TopoMap {
    fn new() -> Self {
        Self {
            map: Vec::new(),
            dim: Vec2::new(0,0),
            start: 0,
            dest: 0
        }
    }

    fn get_val(&self, loc: Vec2) -> Option<i32> {
        self.map.get((loc.y * self.dim.x) + loc.x).copied()
    }

    fn get_index(&self, loc: Vec2) -> usize {
        (loc.y * self.dim.x) + loc.x
    }

    fn get_pos(&self, i: usize) -> Vec2 {
        Vec2 { x: i % self.dim.x, y: i / self.dim.x }
    }

    fn is_in(&self, loc: Vec2) -> bool {
        loc.x < self.dim.x && loc.y < self.dim.y
    }

    fn print(&self) {
        for y in 0..self.dim.y {
            for x in 0..self.dim.x {
                print!("{} ", self.get_val(Vec2{x:x,y:y}).unwrap());
            }
            print!("\n");
        }
    }
}

impl FromStr for TopoMap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = TopoMap::new();

        let map_w = s.find("\r\n");

        if map_w.is_none() {
            return Err("Failed to parse map width");
        }

        let map_w = map_w.unwrap();

        let lines: Vec<_> = s.split("\r\n").collect();

        let map_h = lines.len();
        out.dim = Vec2::new(map_w, map_h);

        let mut fail = false;
        let mut i = 0;
        lines.iter().for_each(|l| {
            l.chars().for_each(|c| {
                if c == 'S' {
                    out.map.push(0);
                    out.start = i;
                }
                else if c == 'E' { 
                    out.map.push(25);
                    out.dest = i;
                }
                else {
                    if let Some(x) = c.to_digit(36) {
                        let x_i: i32 = x.try_into().unwrap();
                        out.map.push(x_i - 10);
                    } else {
                        fail = true;
                    }
                }

                i += 1;
            });
        });

        if fail {
            return Err("Failed to parse map");
        }

        Ok(out)
    }
}

fn find_shortest_path(map: &TopoMap, finished: fn(usize, &TopoMap) -> bool) -> Option<i32> {
    let mut distances = HashMap::new();
    let mut to_explore = VecDeque::new();

    distances.insert(map.dest, 0);
    to_explore.push_back(map.dest);

    while to_explore.len() != 0 {
        let curr_i = to_explore.pop_front().unwrap();
        let curr = map.get_pos(curr_i);
        let curr_dist = distances.get(&curr_i).unwrap().clone();

        if finished(curr_i, map) {
            return Some(curr_dist);
        }
        
        [Dir::North, Dir::East, Dir::South, Dir::West].iter().for_each(|dir| {
            if let Some(step) = curr.step(*dir) {
                let step_i = map.get_index(step);

                if !map.is_in(step) {
                    return;
                }

                let old_v = map.get_val(curr).unwrap();
                let new_v = map.get_val(step).unwrap();

                if old_v - new_v > 1 {
                    return;
                }

                if !distances.contains_key(&step_i) {
                    to_explore.push_back(step_i);
                    distances.insert(step_i, curr_dist + 1);
                }
            }
        });
    }
    None
}

pub fn day12() {
    let input = include_str!("input/day12.txt");

    let map = TopoMap::from_str(input).unwrap();

    println!("Day 12");

    let p1 = find_shortest_path(&map, |i, m| { i == m.start});
    println!("Part 1: Shortest Distance = {}", p1.unwrap());

    let p2 = find_shortest_path(&map, |i, m| { m.get_val(m.get_pos(i)).unwrap() == 0});
    println!("Part 2: Shortest Distance = {}", p2.unwrap());
}
