use aoc_tools::input;

pub(crate) fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let input = input::read_input(2015, 8);

    let (code_counts, memory_counts) =
        input
            .lines()
            .fold((0, 0), |(code_count, memory_count), line| {
                (code_count + line.len(), memory_count + memory_len(line))
            });

    code_counts - memory_counts
}

fn part2() -> usize {
    let input = input::read_input(2015, 8);

    let (code_counts, encoded_counts) =
        input.lines().fold((0, 0), |(code_count, encoded), line| {
            (code_count + line.len(), encoded + encoded_len(line))
        });

    encoded_counts - code_counts
}

fn memory_len(string: &str) -> usize {
    let mut chars = string.chars();
    let mut count = 0;

    if chars.next() != Some('"') {
        panic!("Invalid string format. Expected string to start with '\"'.");
    }

    while let Some(chr) = chars.next() {
        match chr {
            '\\' => match chars.next() {
                Some('x') => {
                    let digits = chars.by_ref().take(2).collect::<Vec<char>>();
                    if digits.len() == 2 && digits.iter().all(|c| c.is_ascii_hexdigit()) {
                        count += 1;
                    } else {
                        panic!("Invalid string format. Unrecognised hex escape sequence.");
                    }
                }
                Some(_) => count += 1,
                None => panic!(
                    "Invalid string format. Orphaned escape '\\' character at the end of the string."
                ),
            },
            '"' => break,
            _ => count += 1,
        };
    }
    count
}

fn encoded_len(string: &str) -> usize {
    string.chars().fold(2, |count, chr| {
        count
            + match chr {
                '"' | '\\' => 2,
                _ => 1,
            }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_simple_empty_strings() {
        assert_eq!(memory_len(r#""""#), 0);
    }

    #[test]
    fn it_counts_simple_strings() {
        assert_eq!(memory_len(r#""abc""#), 3);
    }

    #[test]
    fn it_counts_strings_with_all_escapes() {
        assert_eq!(memory_len(r#""\\\"\x41""#), 3);
    }

    #[test]
    fn it_counts_encoded_empty_strings() {
        assert_eq!(encoded_len(r#""""#), 6);
    }

    #[test]
    fn it_counts_encoded_simple_strings() {
        assert_eq!(encoded_len(r#""abc""#), 9);
    }

    #[test]
    fn it_counts_encoded_escaped_strings() {
        assert_eq!(encoded_len(r#""aaa\"aaa""#), 16);
    }

    #[test]
    fn it_counts_encoded_escaped_hex() {
        assert_eq!(encoded_len(r#""\x27""#), 11);
    }
}
