use std::fs::File;
use std::io::Read;

pub fn run_day1() {
    println!("Starting day 1!");

    let mut f = File::open("data/day1.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let elves = s.split("\n\n").map(
        |string_elf| { 
            string_elf.split_whitespace().map(
                |string_weight| { string_weight.parse::<i32>().expect("Unable to parse int")}).collect::<Vec<_>>() }).collect::<Vec<_>>();

    let mut weights = elves.iter().map(|weights| { weights.iter().sum::<i32>() }).collect::<Vec<_>>();
    weights.sort_by_key(|w| -w);

    let max = weights.iter().max().expect("No weights found");

    println!("Max: {}", max);


    let max3 = weights.iter().take(3).sum::<i32>();

    println!("Top 3: {}", max3);
}