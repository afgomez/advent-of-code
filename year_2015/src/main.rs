mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    // Read the first argument and parse it as a number
    let number = std::env::args()
        .nth(1)
        .expect("Please provide a number as an argument");
    let number = number
        .parse::<i32>()
        .expect("The provided argument is not a valid number");

    match number {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),
        _ => println!("No solution found"),
    }
}
