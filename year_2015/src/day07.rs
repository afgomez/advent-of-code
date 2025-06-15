use aoc_tools::input;
use std::collections::HashMap;

pub(crate) fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u16 {
    let input = input::read_input(2015, 7);
    let mut circuit = Circuit::new();
    for line in input.lines() {
        circuit.connect(line);
    }

    circuit.measure("a")
}

fn part2() -> u16 {
    let input = input::read_input(2015, 7);
    let mut circuit = Circuit::new();
    for line in input.lines() {
        circuit.connect(line);
    }

    // PERF: If/when benchmarking, I can hardcode this value.
    circuit.connect(format!("{} -> b", part1()).as_str());
    circuit.measure("a")
}

#[derive(Debug, PartialEq, Clone)]
enum Input {
    Wire(String),
    Signal(u16),
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        if let Ok(signal) = input.parse() {
            Input::Signal(signal)
        } else {
            Input::Wire(input.to_string())
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Set(Input),
    Not(Input),
    And(Input, Input),
    Or(Input, Input),
    LShift(Input, u16),
    RShift(Input, u16),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let tokens = input.split_whitespace().collect::<Vec<&str>>();

        match tokens.as_slice() {
            [input] => Instruction::Set(Input::from(*input)),
            ["NOT", input] => Instruction::Not(Input::from(*input)),
            [lhs, "AND", rhs] => Instruction::And(Input::from(*lhs), Input::from(*rhs)),
            [lhs, "OR", rhs] => Instruction::Or(Input::from(*lhs), Input::from(*rhs)),
            [lhs, "LSHIFT", rhs] => {
                Instruction::LShift(Input::from(*lhs), rhs.parse().expect("Parse error"))
            }

            [lhs, "RSHIFT", rhs] => {
                Instruction::RShift(Input::from(*lhs), rhs.parse().expect("Parse error"))
            }

            _ => panic!("Parse error"),
        }
    }
}

struct Circuit {
    connections: HashMap<String, Instruction>,
    cache: HashMap<String, u16>,
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            connections: HashMap::new(),
            cache: HashMap::new(),
        }
    }

    fn connect(&mut self, connection: &str) {
        let (instruction, wire) = connection.split_once(" -> ").expect("Parse error");

        let instruction = Instruction::from(instruction);

        if let Instruction::Set(Input::Signal(signal)) = instruction {
            self.cache.insert(wire.to_string(), signal);
        }

        self.connections.insert(wire.to_string(), instruction);
    }

    fn measure(&mut self, wire: &str) -> u16 {
        if let Some(signal) = self.cache.get(wire) {
            *signal
        } else {
            self.walk(wire)
        }
    }

    fn walk(&mut self, wire: &str) -> u16 {
        if let Some(instruction) = self.connections.get(wire) {
            let value = match instruction.clone() {
                Instruction::Set(input) => self.read_input(input),
                Instruction::Not(input) => !self.read_input(input),
                Instruction::And(left, right) => self.read_input(left) & self.read_input(right),
                Instruction::Or(left, right) => self.read_input(left) | self.read_input(right),
                Instruction::LShift(left, right) => self.read_input(left) << right,
                Instruction::RShift(left, right) => self.read_input(left) >> right,
            };

            self.cache.insert(wire.to_string(), value);
            value
        } else {
            panic!("Tried to read from an unknown wire: {}", &wire);
        }
    }

    fn read_input(&mut self, input: Input) -> u16 {
        match input {
            Input::Signal(signal) => signal,
            Input::Wire(wire) => self.measure(&wire),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_inputs() {
        assert_eq!(Input::from("a"), Input::Wire("a".to_string()));
        assert_eq!(Input::from("1234"), Input::Signal(1234));
    }

    #[test]
    fn it_parses_instructions() {
        assert_eq!(
            Instruction::from("a"),
            Instruction::Set(Input::Wire("a".to_string()))
        );
        assert_eq!(
            Instruction::from("NOT a"),
            Instruction::Not(Input::Wire("a".to_string()))
        );
        assert_eq!(
            Instruction::from("a AND b"),
            Instruction::And(Input::Wire("a".to_string()), Input::Wire("b".to_string()))
        );
        assert_eq!(
            Instruction::from("a OR b"),
            Instruction::Or(Input::Wire("a".to_string()), Input::Wire("b".to_string()))
        );
        assert_eq!(
            Instruction::from("a LSHIFT 2"),
            Instruction::LShift(Input::Wire("a".to_string()), 2)
        );
        assert_eq!(
            Instruction::from("a RSHIFT 3"),
            Instruction::RShift(Input::Wire("a".to_string()), 3)
        );
    }

    #[test]
    fn it_connects_everything() {
        let mut circuit = Circuit::new();
        circuit.connect("123 -> x");
        circuit.connect("456 -> y");
        circuit.connect("x AND y -> d");
        circuit.connect("x OR y -> e");
        circuit.connect("x LSHIFT 2 -> f");
        circuit.connect("y RSHIFT 2 -> g");
        circuit.connect("NOT x -> h");
        circuit.connect("NOT y -> i");

        assert_eq!(circuit.measure("d"), 72);
        assert_eq!(circuit.measure("e"), 507);
        assert_eq!(circuit.measure("f"), 492);
        assert_eq!(circuit.measure("g"), 114);
        assert_eq!(circuit.measure("h"), 65412);
        assert_eq!(circuit.measure("i"), 65079);
        assert_eq!(circuit.measure("x"), 123);
        assert_eq!(circuit.measure("y"), 456);
    }
}
