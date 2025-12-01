use std::fs::File;
use std::io::BufRead;

fn path(day: usize) -> String {
    format!("/home/tedem/dev/RustroverProjects/aoc_2025/input/{day}.txt")
}

pub fn read_lines(day: usize) -> impl Iterator<Item = String> {
    File::open_buffered(path(day)).expect("Failed to open file").lines().map_while(Result::ok)
}