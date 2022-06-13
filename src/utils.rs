use std::io;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

pub fn num_parse(input: &str) -> u64 {
    input
        .trim()
        .parse()
        .expect("Failed to parse numeric string")
}
