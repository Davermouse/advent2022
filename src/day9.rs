use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use scan_fmt::scan_fmt;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vector {
    x: i32,
    y: i32
}

impl Vector {
    fn add(self, other: &Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y }
    }

    fn dist(self, other: &Vector) -> u32 {
        return i32::abs_diff(self.x, other.x) + i32::abs_diff(self.y, other.y)
    }
}

struct Move {
    direction: char,
    distance: u32,
}

fn parse_move(row: &str) -> Move {
    if let Ok((direction, distance)) = scan_fmt!(row, "{} {d}", char, u32) {
        Move {
            direction,
            distance
        }
    } else {
        panic!("Unable to parse row: {}", row);
    }
}

fn move_rope(rope: &Vec<Vector>, dir: char) -> Vec<Vector> {
    let mut updated_rope = vec!();

    let d = match dir {
        'R' => Vector { x: 1, y: 0 },
        'L' => Vector { x: -1, y: 0 },
        'U' => Vector { x: 0, y: 1 },
        'D' => Vector { x: 0, y: -1 },
        _ => panic!("Unknown direction")
    };

    let mut last_segment = rope.first().expect("Rope must have a start").add(&d);

    updated_rope.push(last_segment);
    
    for tail_segment in rope.iter().skip(1) {
        let mut updated_segment = tail_segment.clone();

        // If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough
        if 
            (last_segment.x == updated_segment.x && i32::abs_diff(last_segment.y, updated_segment.y) > 1) ||
            (last_segment.y == updated_segment.y && i32::abs_diff(last_segment.x, updated_segment.x) > 1)
        {
            updated_segment = updated_segment.add(&Vector { 
                x: if last_segment.x < updated_segment.x { -1 } else if last_segment.x > updated_segment.x { 1 } else { 0 }, 
                y: if last_segment.y < updated_segment.y { -1 } else if last_segment.y > updated_segment.y { 1 } else { 0 }
            });
        } else if last_segment.dist(&updated_segment) > 2 {
            // Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:

            updated_segment = updated_segment.add(
                &Vector {
                    x: if updated_segment.x < last_segment.x { 1 } else { -1 },
                    y: if updated_segment.y < last_segment.y { 1 } else { -1 },
                }
            );
        }

        updated_rope.push(updated_segment);
        last_segment = updated_segment;
    }

    return updated_rope;
}

fn run_moves(moves: &Vec<Move>, length: i32) -> i32 {
    let mut rope = vec!();

    for _ in 0..length {
        rope.push(Vector { x: 0, y: 0 })
    }

    let mut visited_locations = HashSet::new();
    visited_locations.insert(Vector { x: 0, y: 0 });

    for m in moves.iter() {
        for _ in 0..m.distance {
            let updated_rope = move_rope(&rope, m.direction);

            visited_locations.insert(*updated_rope.last().expect("Rope must have an end"));
            
            rope = updated_rope;
        }
    }

    return visited_locations.len() as i32;
}

pub fn run_day9() {
    println!("Starting day 9!");

    let mut f = File::open("data/day9.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let moves = s.split("\n").map(
        |row| { parse_move(row) }).collect::<Vec<_>>();

    println!("Loaded moves: {}", moves.len());

    let moves_2 = run_moves(&moves, 2);

    println!("Length 2 visited locations: {}", moves_2);

    let moves_10 = run_moves(&moves, 10);

    println!("Length 10 visited locations: {}", moves_10);
}

mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let mut rope = vec!(
            Vector { x: 0, y: 0 },
            Vector { x: 0, y: 0 }
        );

        assert_eq!(
            super::move_rope(
                &rope, 
                'U'
            ),
            vec!(
                Vector { x: 0, y: 1 },
                Vector { x: 0, y: 0 }
            )
        );
    }
}