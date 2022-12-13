mod elf;

use elf::Elf;

pub fn day1() {
    let input = include_str!("input/day1.txt");

    let elf_foods: Vec<_> = input.split("\r\n\r\n").collect();

    let mut elves: Vec<Elf> = Vec::new();

    for elf_food in elf_foods {
        let food : Vec<i32> = elf_food.split("\r\n").map(|s| s.parse().unwrap_or(-1)).collect();

        elves.push(Elf::new(food));
    }

    elves.sort_by_key(|e| e.get_total_calories());

    let top_elves: Vec<_> = elves.iter()
                        .rev()
                        .take(3)
                        .collect();

    let total_food = top_elves.iter()
                                .map(|e| e.get_total_calories())
                                .sum::<i32>();

    println!("Day 1");
    println!("Part 1: Top Elf's food = {}", top_elves.get(0).unwrap().get_total_calories());
    println!("Part 2: Total food = {}", total_food)
}