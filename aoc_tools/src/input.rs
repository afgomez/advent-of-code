
/// Read an input for a specific year and day
pub fn read_input(year: u32, day: u32) -> String {
    let input_path = format!("inputs/{year}/{day:02}.txt");
    std::fs::read_to_string(input_path).expect("Failed to read input")
}
