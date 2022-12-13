pub fn day3() {
    let input = include_str!("input/day3.txt");

    let rucksacks: Vec<_> = input.split_whitespace().collect();

    let dupes = rucksacks.iter().map(|r| find_dupe(r));
    let prior: u32 = dupes.map(|c| get_priority(&c)).sum();

    println!("Day 3");
    println!("Part 1: Sum of priorities = {}", prior);

    let mut i = 0;
    let mut sum = 0;

    while i < rucksacks.len() - 2 {
        let l1_str = rucksacks.get(i).unwrap();
        let l1: Vec<_> = l1_str.chars().collect();
        i += 1;
        let l2_str = rucksacks.get(i).unwrap();
        let l2: Vec<_> = l2_str.chars().collect();
        i += 1;
        let l3_str = rucksacks.get(i).unwrap();
        let l3: Vec<_> = l3_str.chars().collect();
        i += 1;

        let ints = triple_intersection(&l1, &l2, &l3);

        sum += get_priority(ints.get(0).unwrap());
    }

    println!("Part 2: Sum of priorities = {}", sum)
}

fn find_dupe(rucksack: &str) -> char {
    let len = rucksack.len();

    let left = rucksack.chars().take(len/2).collect::<Vec<_>>();
    let right = rucksack.chars().skip(len/2).collect::<Vec<_>>();

    let dupes = intersection(&left, &right);

    *dupes.get(0).unwrap()
}

fn triple_intersection(l1: &Vec<char>, l2: &Vec<char>, l3: &Vec<char>) -> Vec<char> {
    let i1 = intersection(l1, l2);
    intersection(&i1, l3)
}

fn intersection(l1: &Vec<char>, l2: &Vec<char>) -> Vec<char> {
    let mut dupes = Vec::new();

    l1.iter().for_each(|i| {
        if l2.contains(i) {
            dupes.push(*i);
        }
    });

    dupes
}

fn get_priority(c : &char) -> u32 {
    let p = c.clone() as u32;

    if c.is_ascii_uppercase() {
        p - 38
    }
    else {
        p - 96
    }
}