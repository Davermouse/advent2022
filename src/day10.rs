use std::fs::File;
use std::io::Read;

use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Instruction {
    Nop,
    AddX(i32)
}

fn parse_instruction(row: &str) -> Instruction {
    if row == "noop" {
        return Instruction::Nop;
    }

    if let Ok(arg0) = scan_fmt!(row, "addx {d}", i32) {
        return Instruction::AddX(arg0);
    }

    println!("Unknown instruction: {}", row);

    panic!("Unknown instruction");
}

fn convert_to_microinstructions(program: &Vec<Instruction>) -> Vec<Instruction> {
    program.iter().flat_map(
        |instruction| {
            match instruction {
                Instruction::Nop => vec!(Instruction::Nop),
                Instruction::AddX(arg) => vec!(Instruction::Nop, Instruction::AddX(*arg))
            }
        }
    ).collect_vec()
}

fn execute_program(program: &Vec<Instruction>) -> Vec<i32> {
    let microinstructions = convert_to_microinstructions(program);

    let mut results = Vec::new();

    let mut x = 1;

    results.push(x);

    for micro in microinstructions.iter() {
        match micro {
            Instruction::Nop => {},
            Instruction::AddX(arg) => { x += arg }
        }

        results.push(x);
    }

    //println!("{:?}", (0..).zip(microinstructions.iter().zip(results.iter())).collect::<Vec<_>>());

    return results;
}

fn render_results(results: Vec<i32>) {
    for line in results.iter().array_chunks::<40>() {
        for (sprite_pos, xreg) in (0..).zip(line.iter()) {
            if sprite_pos - 1 <= **xreg && sprite_pos + 1 >= **xreg {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

fn load_program(filename: &str) -> Vec<Instruction> {
    let mut f = File::open(filename).expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    return s.split("\n").map(
        |row| { parse_instruction(row) }).collect::<Vec<_>>();
}

pub fn run_day10() {
    println!("Starting day 10!");
    
    let instructions = load_program("data/day10.txt");
   
    let results = execute_program(&instructions);

    let trace_points = vec!(20, 60, 100, 140, 180, 220);

    let strength_sum: i32 = 
        trace_points
            .iter()
            .map(
            |tp| *tp as i32 * results.get(*tp - 1).expect("Must have executed to trace point"))
            .sum();

    println!("Part 1 strength: {}", strength_sum);

    render_results(results);
}

mod tests {
    #[test]
    fn test_execute_program_small() {
        let program = vec!(
            super::Instruction::Nop,
            super::Instruction::AddX(3),
            super::Instruction::AddX(-5)
        );

        let results = super::execute_program(&program);

        assert_eq!(results, vec!(1, 1, 1, 4, 4, -1));
    }

    #[test]
    fn test_execute_program_full() {
        let program = super::load_program("data/day10_test.txt");
        let results = super::execute_program(&program);

        assert_eq!(*results.get(19).expect(""), 21);
        assert_eq!(*results.get(59).expect(""), 19);
        assert_eq!(*results.get(99).expect(""), 18);
        assert_eq!(*results.get(139).expect(""), 21);
        assert_eq!(*results.get(179).expect(""), 16);
        assert_eq!(*results.get(219).expect(""), 18);
    }
}