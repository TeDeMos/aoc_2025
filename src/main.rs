#![feature(bool_to_result, file_buffered, string_into_chars)]
#![warn(clippy::pedantic)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod utils;

fn main() {
    day5::puzzle1();
    day5::puzzle2();
}
