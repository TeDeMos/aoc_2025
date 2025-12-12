#![feature(bool_to_result, exact_length_collection, file_buffered, string_into_chars)]
#![feature(array_windows)]
#![warn(clippy::pedantic)]

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    day10::puzzle1();
    day10::puzzle2();
}
