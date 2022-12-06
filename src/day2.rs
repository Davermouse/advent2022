use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw
}

fn parse_move(om: &str) -> Move {
    match om {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("Unexpected move"),
    }
}

fn parse_outcome(om: &str) -> Outcome {
    match om {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unexpected move"),
    }
}

fn move_score(m: &Move) -> i32 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    }
}
fn outcome_score(o: &Outcome) -> i32 {
    match o {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6
    }
}

fn score_for_row_part_1(row: &str) -> i32 {
    let c = row.split_whitespace().collect::<Vec<_>>();
    let om = parse_move(c.get(0).expect(""));
    let mm = parse_move(c.get(1).expect(""));

    let mut s = move_score(&mm);

    if om == mm {
        s += 3;
    } else if (mm == Move::Rock && om == Move::Scissors) || 
        (mm == Move::Paper && om == Move::Rock) || 
        (mm == Move::Scissors && om == Move::Paper) {
        s += 6;
    }

    return s;
}

fn score_for_row_part_2(row: &str) -> i32 {
    let c = row.split_whitespace().collect::<Vec<_>>();
    let m = parse_move(c.get(0).expect(""));
    let outcome = parse_outcome(c.get(1).expect(""));

    let r = match outcome {
        Outcome::Draw => m,
        Outcome::Win => match m {
            Move::Paper => Move::Scissors,
            Move::Rock => Move::Paper,
            Move::Scissors => Move::Rock
        },
        Outcome::Lose => match m {
            Move::Paper => Move::Rock,
            Move::Rock => Move::Scissors,
            Move::Scissors => Move::Paper
        }
    };

    return move_score(&r) + outcome_score(&outcome);
}

pub fn run_day2() {
    println!("Starting day 2!");

    let mut f = File::open("data/day2.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let rows = s.split("\n").collect::<Vec<_>>();

    let total_1 = rows.iter().map(|r| score_for_row_part_1(r)).sum::<i32>();

    println!("Part 1 total: {}", total_1);

    let total_2 = rows.iter().map(|r| score_for_row_part_2(r)).sum::<i32>();

    println!("Part 2 total: {}", total_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_y() {
        assert_eq!(score_for_row_part_1("A Y"), 8);
    }
}