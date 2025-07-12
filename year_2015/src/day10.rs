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
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(current_char) = chars.next() {
        let mut count = 1;

        // Count consecutive identical characters
        while chars.peek() == Some(&current_char) {
            chars.next();
            count += 1;
        }

        // Append count and character to result
        result.push_str(&format!("{}{}", count, current_char));
    }

    result
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
