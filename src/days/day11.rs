use std::{str::FromStr};

use num_integer::lcm;
use modpow::modpow;
use num_traits::ToPrimitive;

type WorryLevel = u128;

enum Operation {
    Mult(u128),
    Square,
    Add(u128),
    None
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        if s == "* old" {
            return Ok(Operation::Square);
        }

        let splt = s.split_once(" ");

        if splt.is_none() {
            return Err("Failed to parse operation");
        }

        let (p, x_str) = splt.unwrap();

        let x = x_str.parse::<u128>();

        if x.is_err() {
            return Err("Failed to parse operation param");
        }

        let x = x.unwrap();

        match p {
            "+" => Ok(Operation::Add(x)),
            "*" => Ok(Operation::Mult(x)),
            _ => Err("Failed to parse operation")
        }
    }
}

struct Move {
    dest: usize,
    item: WorryLevel
}

struct Monkey {
    things: Vec<WorryLevel>,
    op: Operation,
    test: WorryLevel,
    true_dest: usize,
    false_dest: usize,
    inspections: u64
}

impl Monkey {
    fn new() -> Self {
        Self { 
            things: Vec::new(),
            op: Operation::None,
            test: 1,
            true_dest: 0,
            false_dest: 0,
            inspections: 0
        }
    }

    fn turn(&mut self) -> Vec<Move> {
        let mut ret = Vec::new();

        let things = self.things.clone();

        things.into_iter().for_each(|thing| {
            self.inspections += 1;
            let mut new_val = thing;

            match self.op {
                Operation::Mult(x) => {
                    new_val *= x;
                }
                Operation::Add(x) => {
                    new_val += x;
                }
                Operation::Square => {
                    new_val = new_val.pow(2);
                }
                _ => {}
            }

            new_val /= 3;            

            if new_val % self.test == 0 {
                ret.push(Move {
                    dest: self.true_dest,
                    item: new_val
                });
            }
            else {
                ret.push(Move {
                    dest: self.false_dest,
                    item: new_val
                })
            }
        });
        
        ret
    }

    fn turn_2(&mut self, lcm: &u128) -> Vec<Move> {
        let mut ret = Vec::new();

        let things = self.things.clone();

        things.into_iter().for_each(|thing| {
            self.inspections += 1;
            let mut new_val = thing;

            match self.op {
                Operation::Mult(x) => {
                    new_val *= x;
                }
                Operation::Add(x) => {
                    new_val += x;
                }
                Operation::Square => {
                    new_val = modpow(&new_val, &2, lcm).to_u128().unwrap();
                }
                _ => {}
            }

            if new_val % self.test == 0 {
                ret.push(Move {
                    dest: self.true_dest,
                    item: new_val
                });
            }
            else {
                ret.push(Move {
                    dest: self.false_dest,
                    item: new_val
                })
            }
        });
        
        ret
    }
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\r\n");
        let mut monkey = Monkey::new();

        if None == lines.next() {
            return Err("Monkey string format error");
        };

        if let Some(l) = lines.next() {
            let (_, right) = l.split_at(18);

            right.split(", ").for_each(|i_str| {
                let i = i_str.parse::<WorryLevel>();

                monkey.things.push(i.unwrap());
            });
        }
        else {
            return Err("Failed to parse items");
        }

        if let Some(l) = lines.next() {
            let (_, right) = l.split_at(23);

            monkey.op = Operation::from_str(right)?;
        }
        else {
            return Err("Failed to parse operation");
        }

        if let Some(l) = lines.next() {
            let (_, right) = l.split_at(21);

            monkey.test = right.parse::<WorryLevel>().unwrap();
        }
        else {
            return Err("Failed to parse test");
        }

        if let Some(l) = lines.next() {
            let (_, right) = l.split_at(29);

            monkey.true_dest = right.parse::<usize>().unwrap();
        }
        else {
            return Err("Failed to parse true destination");
        }

        if let Some(l) = lines.next() {
            let (_, right) = l.split_at(30);

            monkey.false_dest = right.parse::<usize>().unwrap();
        }
        else {
            return Err("Failed to parse true destination");
        }

        return Ok(monkey);
    }
}


pub fn day11() {
    let input = include_str!("input/day11.txt");

    let monkeys_str = input.split("\r\n\r\n");

    let mut monkeys_1: Vec<_> = monkeys_str.clone().map(|m| Monkey::from_str(m).unwrap()).collect();
    let mut monkeys_2: Vec<_> = monkeys_str.map(|m| Monkey::from_str(m).unwrap()).collect();

    for _ in 0..20 {
        for i in 0..monkeys_1.len() {
            let monkey = monkeys_1.get_mut(i).unwrap();

            let moves = monkey.turn();
            monkey.things.clear();

            drop(monkey);

            for m in moves {
                let monkey = monkeys_1.get_mut(m.dest).unwrap();

                monkey.things.push(m.item);
            }
        }
    }

    monkeys_1.sort_by_key(|m|m.inspections);

    let lcm = monkeys_2.iter().fold(1, |acc, m| lcm(acc, m.test));

    for _ in 0..10000 {
        for i in 0..monkeys_2.len() {
            let monkey = monkeys_2.get_mut(i).unwrap();

            let moves = monkey.turn_2(&lcm);
            monkey.things.clear();

            drop(monkey);

            for m in moves {
                let monkey = monkeys_2.get_mut(m.dest).unwrap();

                monkey.things.push(m.item);
            }
        }
    }

    monkeys_2.sort_by_key(|m|m.inspections);

    println!("Day11");
    
    let top = monkeys_1.get(monkeys_1.len() - 1).unwrap();
    let top_second = monkeys_1.get(monkeys_1.len() - 2).unwrap();

    println!("Part 1: Monkey Business = {}", top.inspections * top_second.inspections);
    
    let top = monkeys_2.get(monkeys_2.len() - 1).unwrap();
    let top_second = monkeys_2.get(monkeys_2.len() - 2).unwrap();

    println!("Part 2: Monkey Business = {}", top.inspections * top_second.inspections);
}