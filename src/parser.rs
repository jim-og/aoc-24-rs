use std::fs;

pub fn load_input(day: usize) -> String {
    let path = format!("input/2024/day{}.txt", day);
    fs::read_to_string(path).expect("Unable to open file")
}
