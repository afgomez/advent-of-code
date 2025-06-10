mod day01;

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
        _ => println!("No solution found"),
    }
}
