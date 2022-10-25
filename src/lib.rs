use std::io::stdin;
use std::io::Read;

pub fn run() {
    let mut buffer = [0; 1];

    while stdin().read(&mut buffer).expect("Failed to read line") == 1 && buffer[0] != b'q' {}
}