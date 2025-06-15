use aoc_tools::input;

pub(crate) fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let input = input::read_input(2015, 6);
    let mut grid = GridV1::new();

    for line in input.lines() {
        grid.update(Instruction::from(line));
    }

    grid.count_lights()
}

fn part2() -> u32 {
    let input = input::read_input(2015, 6);
    let mut grid = GridV2::new();

    for line in input.lines() {
        grid.update(Instruction::from(line));
    }

    grid.measure_brightness()
}

#[derive(PartialEq)]
enum Command {
    On,
    Off,
    Toggle,
}

#[derive(PartialEq)]
struct Point(usize, usize);

impl From<&str> for Point {
    fn from(input: &str) -> Self {
        let mut parts = input.split(',');

        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();

        Point(x, y)
    }
}

#[derive(PartialEq)]
struct Instruction {
    command: Command,
    from: Point,
    to: Point,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut parts = input.split(' ');

        let command = match parts.next() {
            Some("turn") => match parts.next() {
                Some("on") => Command::On,
                Some("off") => Command::Off,
                _ => panic!("Invalid command"),
            },
            Some("toggle") => Command::Toggle,
            _ => panic!("Invalid command"),
        };

        let from = Point::from(parts.next().unwrap());
        parts.next(); // skip "through"
        let to = Point::from(parts.next().unwrap());

        Instruction { command, from, to }
    }
}

struct GridV1([[bool; 1000]; 1000]);

impl GridV1 {
    fn new() -> Self {
        GridV1([[false; 1000]; 1000])
    }

    fn update(&mut self, instruction: Instruction) {
        for x in instruction.from.0..=instruction.to.0 {
            for y in instruction.from.1..=instruction.to.1 {
                match instruction.command {
                    Command::On => self.0[x][y] = true,
                    Command::Off => self.0[x][y] = false,
                    Command::Toggle => self.0[x][y] = !self.0[x][y],
                }
            }
        }
    }

    fn count_lights(&self) -> usize {
        self.0.iter().flatten().filter(|&&light| light).count()
    }
}

struct GridV2([[u32; 1000]; 1000]);

impl GridV2 {
    fn new() -> Self {
        GridV2([[0; 1000]; 1000])
    }

    fn update(&mut self, instruction: Instruction) {
        for i in instruction.from.0..=instruction.to.0 {
            for j in instruction.from.1..=instruction.to.1 {
                match instruction.command {
                    Command::On => self.0[i][j] += 1,
                    Command::Off => self.0[i][j] = self.0[i][j].saturating_sub(1),
                    Command::Toggle => self.0[i][j] += 2,
                }
            }
        }
    }

    fn measure_brightness(&self) -> u32 {
        self.0.iter().flatten().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_instructions() {
        assert!(
            Instruction::from("turn on 0,0 through 999,999")
                == Instruction {
                    command: Command::On,
                    from: Point(0, 0),
                    to: Point(999, 999),
                }
        );
        assert!(
            Instruction::from("turn off 0,0 through 999,999")
                == Instruction {
                    command: Command::Off,
                    from: Point(0, 0),
                    to: Point(999, 999),
                }
        );
        assert!(
            Instruction::from("toggle 0,0 through 999,999")
                == Instruction {
                    command: Command::Toggle,
                    from: Point(0, 0),
                    to: Point(999, 999),
                }
        );
    }
}
