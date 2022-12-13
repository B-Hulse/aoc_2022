use std::{vec, cmp::{min, Ordering}};

#[derive(Clone, Eq)]
enum Node {
    Subnode(Vec<Node>),
    Value(i32)
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = self.clone();
        let right = other.clone();

        match (left.clone(), right.clone()) {
            (Node::Value(l), Node::Value(r)) => {
                if l == r {
                    return Ordering::Equal;
                }
                else if l < r {
                    return Ordering::Less;
                }
                else {
                    return Ordering::Greater;
                }
            }
            (Node::Value(_), Node::Subnode(_)) => {
                let left_vec = vec![left];
                let left = Node::Subnode(left_vec);
    
                return left.cmp(&right);
            }
            (Node::Subnode(_), Node::Value(_)) => {
                let right_vec = vec![right];
                let right = Node::Subnode(right_vec);
    
                return left.cmp(&right);
                
            }
            (Node::Subnode(l), Node::Subnode(r)) => {
                let max = min(l.len(), r.len());
    
                for i in 0..max {
                    let res = l.get(i).unwrap().cmp(r.get(i).unwrap());
    
                    if res == Ordering::Equal {
                        continue;
                    }
                    else {
                        return res;
                    }
                }
    
                if l.len() == r.len() {
                    return Ordering::Equal;
                }
                else if l.len() < r.len() {
                    return Ordering::Less;
                }
                else {
                    return Ordering::Greater;
                }
            }
        }
    }
}

enum Token {
    Open,
    Close,
    Comma,
    Value(i32)
}

fn create_node_from_str(s: &str) -> Node {
    let tokens = tokenize(s);
    let mut i = 0;
    create_node(&tokens, &mut i)
}

fn create_node(tokens: &Vec<Token>, i: &mut usize) -> Node {
    let mut subs = Vec::new();

    while *i < tokens.len() {
        let token = tokens.get(*i).unwrap();
        *i += 1;
        
        match token {
            Token::Open => {
                subs.push(create_node(tokens,i));
            }
            Token::Close => {
                return Node::Subnode(subs);
            }
            Token::Comma => {
                continue;      
            }
            Token::Value(v) => {
                subs.push(Node::Value(*v));
            }
        };
    }

    return subs.pop().unwrap();
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut i = 0;
    let mut output = Vec::new();

    let s: Vec<_> = s.chars().collect(); 

    while i < s.len() {
        let token = s.get(i).unwrap();
        
        if *token == '[' {
            i += 1;
            output.push(Token::Open);
        }
        else if *token == ',' {
            i += 1;
            output.push(Token::Comma);
        }
        else if *token == ']' {
            i += 1;
            output.push(Token::Close);
        }
        else {
            let next = s.get(i + 1).unwrap();

            if next.is_digit(10) {
                let mut comb = String::new();
                comb.push(*token);
                comb.push(*next);

                let val = comb.parse::<i32>().unwrap();

                output.push(Token::Value(val));
                i += 2;
            } else {
                let val = token.to_digit(10).unwrap() as i32;

                output.push(Token::Value(val));
                i += 1;

            }
        }
    }

    output
}

fn part1() -> usize {
    let input = include_str!("input/day13.txt");

    let pairs: Vec<_> = input.split("\r\n\r\n").collect();

    let pairs: Vec<_> = pairs.iter().map(|pair| {
        pair.split_once("\r\n").unwrap()
    }).collect();

    let tree_pairs: Vec<_> = pairs.iter().map(|(left, right)| {
        (create_node_from_str(&left), create_node_from_str(&right))
    }).collect();

    let a: Vec<_> = tree_pairs.iter().enumerate().filter_map(|(i, (l, r))| {
        if l < r {
            Some(i+1)
        }
        else {
            None
        }
    }).collect();

    a.iter().sum::<usize>()
}

fn part2() -> usize {
    let input = include_str!("input/day13.txt");

    let mut lines: Vec<_> = input.split("\r\n").collect();

    lines.push("[[2]]");
    lines.push("[[6]]");

    let mut nodes: Vec<_> = lines.iter().filter_map(|l| {
        if *l == "\r\n" || l.is_empty() {
            return None;
        }
        else {
            return Some(create_node_from_str(&l));
        }
    }).collect();

    nodes.sort();

    let divider_1 = create_node_from_str("[[2]]");
    let divider_2 = create_node_from_str("[[6]]");

    let d1_i = nodes.iter().position(|n| *n == divider_1).unwrap();
    let d2_i = nodes.iter().position(|n| *n == divider_2).unwrap();

    (d1_i + 1) * (d2_i + 1)
}

pub fn day13() {
    println!("Day 13");
    println!("Part 1: Sum of ordered pair indexes = {}", part1());
    println!("Part 2: Decoder Key = {}", part2());
}