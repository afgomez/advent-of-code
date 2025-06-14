use std::collections::{HashMap, HashSet};

use aoc_tools::input;

pub(crate) fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let input = input::read_input(2015, 5);
    input.lines().filter(|line| is_nice_v1(line)).count()
}

fn part2() -> usize {
    let input = input::read_input(2015, 5);
    input.lines().filter(|line| is_nice_v2(line)).count()
}

fn is_nice_v1(string: &str) -> bool {
    let mut vowels = 0;
    let mut has_double = false;
    let mut prev_chr: char = '\0';

    for chr in string.chars() {
        // Test for vowels
        if matches!(chr, 'a' | 'e' | 'i' | 'o' | 'u') {
            vowels += 1;
        }

        // Test for double characters
        if chr == prev_chr {
            has_double = true;
        }

        // Test for forbidden strings
        match (prev_chr, chr) {
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return false,
            _ => {}
        }

        prev_chr = chr;
    }

    vowels >= 3 && has_double
}

fn is_nice_v2(string: &str) -> bool {
    let mut has_repeated_pair = false;
    let mut has_letter_sandwich = false;
    let mut pair_indices: HashMap<&str, HashSet<usize>> = HashMap::new();

    for i in 0..string.len() - 1 {
        let pair = &string[i..i + 2];
        let entry = pair_indices.entry(pair).or_insert_with(HashSet::new);

        // The string "aaa" has the pair "aa" at positions 0 and 1, because they
        // overlap. Don't add the current position if it's overlapping with the
        // previous ocurrence.
        // "aaaa" has the pair at positions 0, 1, and 2. 0 and 2 are non
        // overlapping and they need to be treated as valid.
        if !entry.contains(&i.saturating_sub(1)) {
            entry.insert(i);
        }

        if entry.len() >= 2 {
            has_repeated_pair = true;
        }

        if i < string.len() - 2 && string[i..i + 1] == string[i + 2..i + 3] {
            has_letter_sandwich = true;
        }
    }

    has_repeated_pair && has_letter_sandwich
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_is_nice_when_string_has_all_conditions() {
        assert!(is_nice_v1("ugknbfddgicrmopn"));
        assert!(is_nice_v1("aaa"));
    }

    #[test]
    fn v1_is_not_nice_when_string_doesnt_have_all_conditions() {
        assert!(!is_nice_v1("jchzalrnumimnmhp"));
        assert!(!is_nice_v1("haegwjzuvuyypxyu"));
        assert!(!is_nice_v1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn v2_is_nice_when_string_has_all_conditions() {
        assert!(is_nice_v2("xxxx"));
        assert!(is_nice_v2("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn v2_is_not_nice_when_string_doesnt_have_all_conditions() {
        assert!(!is_nice_v2("uurcxstgmygtbstg"));
        assert!(!is_nice_v2("ieodomkazucvgmuy"));
    }
}
