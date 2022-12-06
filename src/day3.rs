use std::fs::File;
use std::io::Read;
use std::iter::ArrayChunks;

fn score_for_item(item: char) -> i32 {
    return "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".find(item).expect("Unknown item") as i32 + 1;
}
fn score_for_part_1(row: &str) -> i32 {
    let (first, second) = row.split_at(row.len() / 2);

    for i in second.chars() {
        let m = first.find(i);

        match m {
            Some(_) => return score_for_item(i),
            None => ()
        }
    }

    panic!("No dupe item found");
}

fn score_for_part_2(a: &str, b: &str, c: &str) -> i32 {
    for i in b.chars() {
        let m1 = a.find(i);

        match m1 {
            Some(_) => {
                let m2 = c.find(i);

                match m2 {
                    Some(_) => return score_for_item(i),
                    None => ()
                }
            }
            None => ()
        }
    }
    
    panic!("No dupe item found");
}

pub fn run_day3() {
    println!("Starting day 3!");

    let mut f = File::open("data/day3.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let rows = s.split("\n").collect::<Vec<_>>();

    let part_1 = rows.iter().map(|r| score_for_part_1(r)).sum::<i32>();

    let part_2 = rows.iter().array_chunks::<3>().map(|[a, b, c] | score_for_part_2(a, b, c)).sum::<i32>();

    println!("Part 1 total: {}", part_1);

    println!("Part 2 total: {}", part_2);
}