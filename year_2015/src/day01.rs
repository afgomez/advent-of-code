use aoc_tools::input;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2().unwrap_or(0));
}

fn part1() -> i32 {
    let input = input::read_input(2015, 1);

    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
    }
    floor
}

fn part2() -> Option<usize> {
    let input = input::read_input(2015, 1);

    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
        if floor == -1 {
            return Some(i + 1);
        }
    }

    None
}
