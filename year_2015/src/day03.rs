use aoc_tools::input;
use std::collections::HashMap;

pub(crate) fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let input = input::read_input(2015, 3);

    let mut house_navigator = HouseNavigator::new(1);
    for instruction in input.chars() {
        house_navigator.navigate(instruction);
    }

    house_navigator.count_houses_with_gifts()
}

fn part2() -> usize {
    let input = input::read_input(2015, 3);

    let mut house_navigator = HouseNavigator::new(2);
    for instruction in input.chars() {
        house_navigator.navigate(instruction);
    }

    house_navigator.count_houses_with_gifts()
}

struct HouseNavigator {
    players: usize,
    turn: usize,
    positions: Vec<(i32, i32)>,
    gifts_per_house: HashMap<(i32, i32), i32>,
}

impl HouseNavigator {
    fn new(players: usize) -> Self {
        let mut gifts_per_house = HashMap::new();
        gifts_per_house.insert((0, 0), 1);
        let positions = vec![(0, 0); players];

        HouseNavigator {
            positions,
            turn: 0,
            players,
            gifts_per_house,
        }
    }

    fn navigate(&mut self, instruction: char) {
        let mut position = self.positions[self.turn];

        match instruction {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            _ => {} // Ignore any other characters
        }

        *self.gifts_per_house.entry(position).or_insert(0) += 1;

        self.positions[self.turn] = position;
        self.turn = (self.turn + 1) % self.players;
    }

    fn count_houses_with_gifts(&self) -> usize {
        self.gifts_per_house.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_at_least_one_house() {
        let house_navigator = HouseNavigator::new(1);
        assert_eq!(house_navigator.count_houses_with_gifts(), 1);
    }

    #[test]
    fn adds_one_house_after_navigation() {
        let mut house_navigator = HouseNavigator::new(1);
        house_navigator.navigate('^');
        assert_eq!(house_navigator.count_houses_with_gifts(), 2);
    }

    #[test]
    fn circles_four_houses() {
        let mut house_navigator = HouseNavigator::new(1);
        house_navigator.navigate('^');
        house_navigator.navigate('>');
        house_navigator.navigate('v');
        house_navigator.navigate('<');
        assert_eq!(house_navigator.count_houses_with_gifts(), 4);
    }

    #[test]
    fn supports_multiple_players() {
        let mut house_navigator = HouseNavigator::new(2);
        house_navigator.navigate('^');
        house_navigator.navigate('v');
        assert_eq!(house_navigator.count_houses_with_gifts(), 3);
    }
}
