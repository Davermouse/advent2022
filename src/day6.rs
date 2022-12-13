use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn find_marker(s: &String, len: usize) -> usize {
    for i in len..s.len() {
        let mut chars:HashSet<char> = HashSet::new();

        for c in s.chars().skip(i - len).take(len) {
            chars.insert(c);
        }

        if chars.len() == len {
            return i;
        }
    }

    panic!("No marker found");
}

pub fn run_day6() {
    println!("Starting day 6!");

    let mut f = File::open("data/day6.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let marker_idx = find_marker(&s, 4);
    
    println!("Part 1: {}", marker_idx);

    let message_idx = find_marker(&s, 14);
    
    println!("Part 2: {}", message_idx);
}

mod tests {
    use super::*;

    #[test]
    fn test_find_marker() {
        assert_eq!(super::find_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 4), 7);
    }
}