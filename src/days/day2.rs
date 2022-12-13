pub fn day2() {
    let input = include_str!("input/day2.txt");

    let rounds = input.split("\r\n")
                                    .map( |v| 
                                        v.split_whitespace().collect::<Vec<_>>()
                                    )
                                    .collect::<Vec<_>>();

    let mut score_1 = 0;
    let mut score_2 = 0;

    for round in rounds {
        score_1 += compete_1(round.get(0).unwrap(), 
                        round.get(1).unwrap());

        score_2 += compete_2(round.get(0).unwrap(), 
                        round.get(1).unwrap());
    }

    println!("Day 2");
    println!("Part 1: Score with original strategy = {}", score_1);
    println!("Part 2: Score with real strategy = {}", score_2)
}

fn compete_1(theirs: &str, mine: &str) -> i32 {
    let mut score = match mine{
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _=> 0,
    };

    score += match (theirs, mine) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _=> 0,
    };

    score
}


fn compete_2(theirs: &str, mine: &str) -> i32 {
    let mut score = match mine{
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _=> 0,
    };

    score += match (theirs, mine) {
        ("A", "X") => 3,
        ("A", "Y") => 1,
        ("A", "Z") => 2,
        ("B", "X") => 1,
        ("B", "Y") => 2,
        ("B", "Z") => 3,
        ("C", "X") => 2,
        ("C", "Y") => 3,
        ("C", "Z") => 1,
        _=> 0,
    };

    score
}