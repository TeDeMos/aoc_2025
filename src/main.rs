#![feature(bool_to_result, exact_length_collection, file_buffered, string_into_chars)]
#![warn(clippy::pedantic)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod utils;

fn main() {
    day8::puzzle1();
    day8::puzzle2();
}
