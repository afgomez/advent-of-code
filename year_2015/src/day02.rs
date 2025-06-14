use aoc_tools::input;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
    let input = input::read_input(2015, 2);

    input
        .lines()
        .map(|line| {
            let gift = Gift::from(line);
            gift.calculate_paper()
        })
        .sum()
}

fn part2() -> i32 {
    let input = input::read_input(2015, 2);
    input
        .lines()
        .map(|line| {
            let gift = Gift::from(line);
            gift.calculate_ribbon()
        })
        .sum()
}

struct Gift {
    measurements: Vec<i32>,
}
impl Gift {
    fn from(str: &str) -> Gift {
        let mut measurements = str
            .split('x')
            .map(|m| m.parse().unwrap())
            .collect::<Vec<i32>>();
        measurements.sort();
        Gift { measurements }
    }

    fn calculate_paper(&self) -> i32 {
        let l = self.measurements[0];
        let w = self.measurements[1];
        let h = self.measurements[2];
        let (side1, side2, side3) = (l * w, w * h, h * l);

        2 * side1 + 2 * side2 + 2 * side3 + side1.min(side2).min(side3)
    }

    fn calculate_ribbon(&self) -> i32 {
        let ribbon = self.measurements.iter().take(2).sum::<i32>() * 2;
        let bow = self.measurements.iter().product::<i32>();
        ribbon + bow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_paper_correctly() {
        let gift = Gift::from("2x3x4");
        assert_eq!(gift.calculate_paper(), 58);
    }

    #[test]
    fn calculates_ribbon_correctly() {
        let gift = Gift::from("2x3x4");
        assert_eq!(gift.calculate_ribbon(), 34);
    }
}
