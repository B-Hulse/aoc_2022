use std::{str::FromStr};

enum Operation {
    Noop,
    Addx(i32)
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        match parts.next() {
            Some("noop") => {
                return Ok(Operation::Noop);
            }
            Some("addx") => {
                if let Ok(x) = parts.next().unwrap_or("").parse::<i32>() {
                    return Ok(Operation::Addx(x));
                }
            }
            _ => {return Err("Failed to parse operation");}
        };

        return Err("");
    }
}
struct Machine {
    register: i32,
    cycle: i32,
    strength_sum: i32,
    display: String,
    display_width: usize
}

impl Machine {
    fn new(w: usize) -> Self {
        Self {
            register: 1,
            cycle: 0,
            strength_sum: 0,
            display: String::new(),
            display_width: w
        }
    }

    fn increment(&mut self) {
        self.cycle += 1;

        let i = (self.cycle - 1) % self.display_width as i32;

        if i == 0 && self.cycle != 1 {
            self.display.push('\n');
        }

        if (self.register - i).abs() <= 1 {
            self.display.push('#');
        }
        else {
            self.display.push('.');
        }

        if (self.cycle % 40) - 20 == 0 {
            self.strength_sum += self.register * self.cycle;
        }
    }

    fn run(&mut self, op: &Operation) {
        match op {
            Operation::Noop => {
                self.noop();
            }
            Operation::Addx(x) => {
                self.addx(*x);
            }
        }
    }

    fn noop(&mut self) {
        self.increment();
    }

    fn addx(&mut self, x: i32) {
        self.increment();
        self.increment();
        self.register += x;
    }

    fn get_sum(&self) -> i32 {
        self.strength_sum
    }
}


pub fn day10() {
    let input = include_str!("input/day10.txt");

    let commands: Vec<_> = input.split("\r\n").map(|s| Operation::from_str(s)).collect();

    let mut machine = Machine::new(40);

    for c in commands {
        if let Ok(op) = c {
            machine.run(&op);
        } 
    }

    println!("Day 10");
    println!("Part 1: Sum of strenghts = {}", machine.get_sum());

    println!{"Part 2:\n{}", machine.display};
}