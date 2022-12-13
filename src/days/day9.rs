use std::{collections::HashSet, str::FromStr, ops::{Add, AddAssign}};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Vec2d {
    x: i32,
    y: i32
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct Instruction {
    dir: Vec2d,
    dist: i32
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, dist_str) = s.split_at(1);

        let dir_ = match dir_str {
            "R" => Vec2d {x: 1, y: 0},
            "L" => Vec2d { x: -1, y: 0 },
            "U" => Vec2d { x: 0, y: 1 },
            "D" => Vec2d { x: 0, y: -1 },
            _ => {return Err("Not valid direction")}
        };

        if let Ok(dist_) = dist_str.trim().parse::<i32>() {
            return Ok(Instruction {dir: dir_, dist: dist_});
        }
        else {
            return Err("Could not parse distance");
        }
    }
}

struct Knot {
    pos: Vec2d,
    next_offset: Vec2d
}

impl Knot {
    fn new() -> Self {
        Self { pos: Vec2d{x:0,y:0}, next_offset: Vec2d{x:0,y:0} }   
    }

    fn add_offset(&mut self, o: Vec2d) -> Vec2d {
        self.next_offset += o;

        let mut delta = Vec2d { x: 0, y: 0 };

        if self.next_offset.x.abs() > 1 || self.next_offset.y.abs() > 1 {
            delta = Vec2d{x:0, y:0};

            if self.next_offset.x < 0 {
                delta.x = -1;
                self.next_offset.x += 1;
            }
            if self.next_offset.x > 0 { 
                delta.x = 1;
                self.next_offset.x -= 1;
            }
            if self.next_offset.y < 0 {
                delta.y = -1;
                self.next_offset.y += 1;
            }
            if self.next_offset.y > 0 {
                delta.y = 1;
                self.next_offset.y -= 1;
            }

            self.pos += delta;
        }

        delta
    }
}

struct Rope {
    knots: Vec<Knot>,
    tail_visited: HashSet<Vec2d>
}

impl Rope {
    fn new(size: usize) -> Self {
        let mut k = Vec::new();

        for _ in 0..size {
            k.push(Knot::new());
        }

        Self { 
            knots: k,
            tail_visited: HashSet::new()
        }
    }

    fn tail(&self) -> &Knot {
        self.knots.last().unwrap()
    }

    fn move_head(&mut self, d: Vec2d) {
        let mut last_delta = self.knots.get_mut(0).unwrap().add_offset(d);
        for k in self.knots.iter_mut().skip(1) {
            last_delta = k.add_offset(last_delta);
        }
        self.tail_visited.insert(self.tail().pos.clone());
    }
}

pub fn day9() {
    let input = include_str!("input/day9.txt");

    let moves: Vec<_> = input.split("\r\n").map(|l| {
        Instruction::from_str(l).unwrap()
    }).collect();

    let mut rope = Rope::new(1);
    let mut rope_long = Rope::new(9);

    for m in moves {
        for _ in 0..m.dist {
            rope.move_head(m.dir);
            rope_long.move_head(m.dir);
        }
    }

    println!("Day 9");
    println!("Part 1: Tail visited {} places", rope.tail_visited.len());
    println!("Part 2: Tail visited {} places", rope_long.tail_visited.len());
}