#![feature(bool_to_result, exact_length_collection, file_buffered, string_into_chars)]
#![feature(array_windows)]
#![feature(result_option_map_or_default)]
#![warn(clippy::pedantic)]

mod day1;
mod day10;
mod day11;
mod day12;
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
    day1::puzzle1();
    day1::puzzle2();
    day2::puzzle1();
    day2::puzzle2();
    day3::puzzle1();
    day3::puzzle2();
    day4::puzzle1();
    day4::puzzle2();
    day5::puzzle1();
    day5::puzzle2();
    day6::puzzle1();
    day6::puzzle2();
    day7::puzzle1();
    day7::puzzle2();
    day8::puzzle1();
    day8::puzzle2();
    day9::puzzle1();
    day9::puzzle2();
    day10::puzzle1();
    day10::puzzle2();
    day11::puzzle1();
    day11::puzzle2();
    day12::puzzle1();
}
