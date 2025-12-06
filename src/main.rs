#![feature(bool_to_result, file_buffered, string_into_chars)]
#![warn(clippy::pedantic)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod utils;

fn main() {
    day6::puzzle1();
    day6::puzzle2();
}
