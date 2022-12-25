use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use itertools::Itertools;
use scan_fmt::scan_fmt;

struct Monkey {
    name: String,
    value: Option<i64>,
    operation_type: char,
    param_a: String,
    param_b: String
}

impl Monkey {
    fn from_str(line: &str) -> Self {
        if let Ok((name, c)) = scan_fmt!(line, "{}: {d}", String, i64) {
            return Monkey {
                name,
                value: Some(c),
                operation_type: 'c',
                param_a: "".to_string(),
                param_b: "".to_string()
            };
        }
        
        if let Ok((name, arg_a, op, arg_b)) = scan_fmt!(line, "{}: {} {} {}", String, String, char, String) {
            return Monkey {
                name,
                value: None,
                operation_type: op,
                param_a: arg_a,
                param_b: arg_b
            };
        }
    
        println!("{}", line);
        panic!("Unable to parse line");
    }
}

fn load_monkeys(filename: &str) -> HashMap<String, Monkey> {
    let mut f = File::open(filename).expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let mut monkeys = HashMap::<String, Monkey>::new();

    for line in s.lines() {
        let monkey = Monkey::from_str(line);

        monkeys.insert(monkey.name.clone(), monkey);
    }

    println!("Loaded {} monkeys", monkeys.len());

    return monkeys;
}

fn resolve_monkey(name: String, monkeys: &HashMap<String, Monkey>) -> i64 {
    let monkey = monkeys.get(&name).expect("Unable to find monkey");

    if let Some(result) = monkey.value {
        return result;
    }

    let val_a = resolve_monkey(monkey.param_a.clone(), &monkeys);
    let val_b = resolve_monkey(monkey.param_b.clone(), &monkeys);

    match monkey.operation_type {
        '+' => val_a + val_b,
        '-' => val_a - val_b,
        '*' => val_a * val_b,
        '/' => val_a / val_b,
        _ => panic!("Unexpected operation")
    }
}

fn compute_humn(mut monkeys: HashMap<String, Monkey>) -> i64 {
    let root = monkeys.get("root".clone()).expect("unable to find root");
    
    let root_left = root.param_a.clone();
    let root_right = root.param_b.clone();

    for i in 1000000..2000000 {
        let humn = monkeys.get_mut("humn").expect("unable to find humn");

        humn.value = Some(i);

        let val_left = resolve_monkey(root_left.clone(), &monkeys);
        let val_right = resolve_monkey(root_right.clone(), &monkeys);

        if val_left == val_right {
            return i;
        }
    }

    panic!("unable to find humn");
}

pub fn run_day21() {
    println!("Starting day 21!");

    let mut monkeys = load_monkeys("data/day21.txt");

    let result = resolve_monkey("root".to_string(), &monkeys);

    print!("Part 1 result: {}", result);

    let humn = compute_humn(monkeys);

    print!("Humn: {}", humn);
}

mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let cave = load_cave("data/day16_test.txt");
    }
}
