#![feature(bool_to_result, exact_length_collection, file_buffered, string_into_chars)]
#![feature(array_windows)]
#![feature(result_option_map_or_default)]
#![warn(clippy::pedantic)]

mod day1;
mod day10;
mod day11;
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
    day11::puzzle1();
    day11::puzzle2();
}
