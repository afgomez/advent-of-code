use aoc_tools::input::read_input;

pub(crate) fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let mut input = read_input(2015, 10).trim().to_owned();
    for _ in 0..40 {
        input = look_and_say(&input);
    }

    input.len()
}

fn part2() -> usize {
    let mut input = read_input(2015, 10).trim().to_owned();
    for _ in 0..50 {
        input = look_and_say(&input);
    }

    input.len()
}

fn look_and_say(input: &str) -> String {
    let mut output = String::new();
    let mut cursor = 0;

    let chars: Vec<char> = input.chars().collect();

    while cursor < chars.len() {
        let c = chars.get(cursor).unwrap();
        cursor += 1;
        let mut char_count = 1;
        while chars.get(cursor).is_some_and(|next| next == c) {
            cursor += 1;
            char_count += 1;
        }

        output.push_str(format!("{}{}", char_count, c).as_str());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_looks_and_says() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
