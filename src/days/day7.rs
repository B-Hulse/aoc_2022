use std::{ collections::HashMap};

struct Directory {
    size: Option<usize>,
    subs: Vec<String>,
    files: Vec<usize>
}

impl Directory {
    fn new() -> Self {
        Self {
            size: None,
            subs: Vec::new(),
            files: Vec::new()
        }
    }

    fn add_sub(&mut self, name: String) {
        self.subs.push(name);
    }

    fn get_size(& self, dirs: &HashMap<String,Directory>) -> usize {
        if self.size.is_some() {
            return self.size.unwrap();
        }

        let mut sum: usize = 0;

        self.subs.iter().for_each(|s| {
            sum += dirs.get(s).unwrap().get_size(dirs);
        });
        
        sum + self.files.iter().sum::<usize>()
    }
}

fn vtp(vec : &Vec<String>) -> String {
    vec.iter().fold(String::new(), |mut acc, s| {
        acc.push_str(&s); 
        acc.push('/'); 
        acc
    })
}

pub fn day7() {
    let input = include_str!("input/day7.txt");
    let mut lines = input.split("\r\n");

    let mut dirs: HashMap<String, Directory> = HashMap::new();

    let l1 = lines.next();
    assert!(l1 == Some("$ cd /"));

    let mut c_path:Vec<String> = vec!["/".to_string()];

    dirs.insert(vtp(&c_path), Directory::new());

    lines.for_each(|line| {
        let tokens = line.split_whitespace().collect::<Vec<_>>();

        match tokens.get(0) {
            Some(&"$") => {
                let cmd = parse_cmd(tokens.get(1).unwrap(), tokens.get(2));
                
                match cmd {
                    Some(Inst::CdBack) => {
                        c_path.pop();
                    }
                    Some(Inst::CdTo(a)) => {
                        let current = dirs.get_mut(&vtp(&c_path)).unwrap();
                        c_path.push(a);
                        current.add_sub(vtp(&c_path));
                        drop(current);

                        dirs.insert(vtp(&c_path), Directory::new());
                    }
                    _ => {
                        return;
                    } 
                }
            }
            _ => {
                if let Some(l) = tokens.get(0) {
                    let current = dirs.get_mut(&vtp(&c_path)).unwrap();
                    if let Ok(size) = l.parse::<usize>() {
                        current.files.push(size);
                        return;
                    }

                    // Ignore directories for now
                }
            }
        }
    });

    println!("Day 7");
    let mut folder_sizes:Vec<(String, usize)> = dirs.iter().map(|(s,d)| (s.clone(), d.get_size(&dirs))).collect();
    let sum: usize = folder_sizes.iter().map(|(_,s)| s).filter(|&s| *s <= 100000).sum();
    println!("Part 1: Sum of small directories = {}", sum);

    let total_space = 70000000;
    let total_used = folder_sizes.iter().find(|(s,_)| *s == "//".to_string()).unwrap().1;

    let total_unused = total_space - total_used;
    let total_needed = 30000000 - total_unused;

    folder_sizes.sort_by_key(|(_,si)| *si);
    if let Some((f_name, f_size)) = folder_sizes.iter().find(|(_,s)| *s > total_needed) {
        println!("Part 2: Should delete folder {} to free up {} of space", f_name, f_size);
    }
    else {
        println!("Part 2: Failed");

    }
}

enum Inst {
    CdBack,
    CdTo(String),
    List
}

fn parse_cmd(cmd: &str, param: Option<&&str>) -> Option<Inst> {
    match cmd {
        "cd" => {
            if param.is_none() {
                None
            }
            else {
                if param.unwrap() == &".." {
                    Some(Inst::CdBack)
                }
                else {
                    Some(Inst::CdTo(param.unwrap().to_string()))
                }
            }
        }

        "ls" => Some(Inst::List),
        _ => None
    }
} 