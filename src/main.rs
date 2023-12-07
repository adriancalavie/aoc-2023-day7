use std::fs;

fn read_lines(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Couldn't read input");
    let lines = content
        .lines()
        .map(|s| -> String { s.trim().to_string() })
        .collect();

    lines
}

fn main() {
    // let lines = read_lines("res/data.txt");
    let lines = read_lines("res/data_light.txt");
    println!("{:?}", lines);
}
