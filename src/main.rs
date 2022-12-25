#![feature(iter_array_chunks)]

mod common;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;


mod day14;

mod day16;




mod day21;

fn main() {
    println!("Starting AoC!");

    day1::run_day1();
    day2::run_day2();
    day3::run_day3();
    day4::run_day4();
    day5::run_day5();
    day6::run_day6();
    day7::run_day7();
    day8::run_day8();
    day9::run_day9();
    day10::run_day10();
//   day11::run_day11();
    day14::run_day14();
    day16::run_day16();
    day21::run_day21();
}
