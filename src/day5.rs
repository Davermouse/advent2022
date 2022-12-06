use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use scan_fmt::scan_fmt;

fn parse_stacks(text: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    let mut rows = text.split("\n").collect::<Vec<_>>();
    rows.reverse();

    let stackCount = rows.get(0).expect("Must have stack names").split_whitespace().count();

    for _ in 0..stackCount {
        stacks.push(Vec::new());
    }

    // [V] [V] [D] [G] [F] [D]     [V] 
    for itemsRow in rows.iter().skip(1) {
        for si in 0..stackCount {
            let item = itemsRow.chars().nth(1 + 4 * si);

            if let Some(i) = item {
                if i != ' ' {
                    let mut stack = stacks.get_mut(si).expect("Missing stack");
                    stack.push(i);
                }
            }
        }
    }

    return stacks;
}

fn parse_moves(text: &str) -> Vec<(usize, usize, usize)> {
    return text.split("\n").map(
        |row|
        if let Ok((x1,y1,x2)) = scan_fmt!(row, "move {d} from {d} to {d}", usize, usize, usize) {
            (x1, y1, x2)
        } else {
            panic!("Unable to parse row: {}", row);
        }).collect::<Vec<_>>();
}

pub fn run_day5() {
    println!("Starting day 5!");

    let mut f = File::open("data/day5.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let main_parts = s.split("\n\n").collect::<Vec<_>>();

    let mut stacks_1 = parse_stacks(main_parts.get(0).expect("Must have stack definitions"));
    let moves = parse_moves(main_parts.get(1).expect("Must have move definitions"));

    for (count, fromIndex, toIndex) in moves.iter() {
        for _ in 0..*count {
            let mut from = stacks_1.get_mut(fromIndex - 1).expect("Missing from stack");

            let item = from.pop().expect("From stack must contain something");

            let mut to = stacks_1.get_mut(toIndex - 1).expect("Missing to stack");

            to.push(item);

        }
    }

    for i in 0..stacks_1.len() {
        print!("{}", stacks_1[i].pop().expect("Stacks must contain things"));
    }
    println!("");

    let mut stacks_2 = parse_stacks(main_parts.get(0).expect("Must have stack definitions"));

    for (count, fromIndex, toIndex) in moves.iter() {
        let mut from = stacks_2.get_mut(*fromIndex - 1).expect("Missing from stack");

        let mut items = Vec::new();

        for _ in 0..*count {
            items.push(from.pop().expect("From must contain items"));
        }

        items.reverse();
        
        let mut to = stacks_2.get_mut(toIndex - 1).expect("Missing to stack");

        for i in items {
            to.push(i);
        }
    }

    for i in 0..stacks_2.len() {
        print!("{}", stacks_2[i].pop().expect("Stacks must contain things"));
    }
    println!("");
}
