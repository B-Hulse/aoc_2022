use std::{collections::{HashSet, VecDeque}};

pub fn day6() {
    let input = include_str!("input/day6.txt");

    let packet_i = find_marker(4, input);
    let message_i = find_marker(14, input);

    println!("Day 6");
    println!("Part 1: Start-of-Packet = {}", packet_i.unwrap() + 1);
    println!("Part 2: Start-of-Message = {}", message_i.unwrap() + 1);
}

fn find_marker(count: usize, message: &str) -> Option<usize> {
    let mut slice = VecDeque::new();
    let mut hash = HashSet::new();

    message.chars().position(|c| {
        slice.push_front(c);
        let unique = hash.insert(c);

        if slice.len() > count {
            let popped = slice.pop_back().unwrap();

            if !slice.contains(&popped) {
                hash.remove(&popped);
            }
        }

        return hash.len() == count && unique;
    })
}

//Alternate method which is less efficient
// fn find_marker(count: usize, message: &str) -> Option<usize> {
//     let mut slice = VecDeque::new();

//     message.chars().position(|c| {
//      slice.push_front(c);

//         if slice.len() > count {
//             slice.pop_back();
//         }

//         return slice.len() == count && unique_slice(&slice);
//     })
// }

// fn unique_slice(slice: &VecDeque<char>) -> bool {
//     let mut hash = HashSet::new();
//     slice.iter().all(|c| hash.insert(c))
// }