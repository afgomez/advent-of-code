use aoc_tools::input;

pub(crate) fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let key = input::read_input(2015, 4);
    let key = key.trim();
    for i in 1.. {
        let coin = format!("{key}{i}");
        let digest = md5::compute(coin.as_bytes());
        // First 5 hex digits are 0
        if digest[0..2] == [0, 0] && digest[2] < 16 {
            return i;
        }
    }
    panic!("No solution found");
}

fn part2() -> usize {
    let key = input::read_input(2015, 4);
    let key = key.trim();
    for i in 1.. {
        let coin = format!("{key}{i}");
        let digest = md5::compute(coin.as_bytes());
        if digest[0..3] == [0, 0, 0] {
            return i;
        }
    }
    panic!("No solution found");
}
