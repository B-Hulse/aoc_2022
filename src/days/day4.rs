pub fn day4() {
    let input = include_str!("input/day4.txt");

    let lines = input.split_whitespace().collect::<Vec<_>>();

    let pairs = lines.iter().map(|l| l.split(",").collect::<Vec<_>>()).collect::<Vec<_>>();

    let contains = pairs.iter().filter(|p| does_contain(p)).collect::<Vec<_>>();
    let overlaps = pairs.iter().filter(|p| does_overlap(p)).collect::<Vec<_>>();

    println!("Day 4");
    println!("Part 1: Number of fully overlapped sets = {}", contains.len());
    println!("Part 1: Number of partially overlapped sets = {}", overlaps.len());
    
}

fn does_contain(p : &Vec<&str>) -> bool {
    let p1 = p.get(0).unwrap().split("-").collect::<Vec<_>>();
    let p2 = p.get(1).unwrap().split("-").collect::<Vec<_>>();

    let l1 = p1.get(0).unwrap().parse::<i32>().unwrap();
    let r1 = p1.get(1).unwrap().parse::<i32>().unwrap();
    let l2 = p2.get(0).unwrap().parse::<i32>().unwrap();
    let r2 = p2.get(1).unwrap().parse::<i32>().unwrap();

    if (l1 <= l2 && r2 <= r1) || (l2 <= l1 && r1 <= r2) {
        return true;
    }

    false
}

fn does_overlap(p : &Vec<&str>) -> bool {
    let p1 = p.get(0).unwrap().split("-").collect::<Vec<_>>();
    let p2 = p.get(1).unwrap().split("-").collect::<Vec<_>>();

    let l1 = p1.get(0).unwrap().parse::<i32>().unwrap();
    let r1 = p1.get(1).unwrap().parse::<i32>().unwrap();
    let l2 = p2.get(0).unwrap().parse::<i32>().unwrap();
    let r2 = p2.get(1).unwrap().parse::<i32>().unwrap();

    if l1 > r2 || r1 < l2 {
            return false;
    }

    true
}