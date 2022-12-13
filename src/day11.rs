use std::fs::File;
use std::io::Read;

use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum OperationArg {
    Constant(i32),
    Old
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Monkey {
    items: Vec<i32>,
    operation_type: char,
    operation_arg: OperationArg,
    test_div_by: i32,
    true_dest: i32,
    false_dest: i32,
    inspections: i64,
}

fn parse_monkey(data: &str) -> Monkey {
    let rows = data.lines().collect_vec();

    let items_string = 
        scan_fmt!(rows.get(1).expect(""), "Starting items: {/.+/}", String).expect("");

    let items = 
        items_string.split(", ")
            .map(|s| i32::from_str_radix(s, 10).expect("Can't parse item")
        ).collect_vec();

    let operation_row = rows.get(2).expect("");
    let operation_type = *operation_row[23..24].chars().collect_vec().get(0).expect("");
    let operation_arg_text =  &operation_row[25..];

    let operation_arg = 
        if operation_arg_text == "old" {
            OperationArg::Old
        } else {
            OperationArg::Constant(operation_arg_text.parse::<i32>().expect("Can't parse operation arg"))
        };
    
    let test_div_by = scan_fmt!(rows.get(3).expect(""), "  Test: divisible by {d}", i32).expect("Can't parse divisible");

    let true_dest = scan_fmt!(rows.get(4).expect(""), "    If true: throw to monkey {d}", i32).expect("Can't parse true dest");
    let false_dest = scan_fmt!(rows.get(5).expect(""), "   If false: throw to monkey {d}", i32).expect("Can't parse true dest");

    return Monkey {
        items,
        operation_type,
        operation_arg,
        test_div_by,
        true_dest,
        false_dest,
        inspections: 0,
    };
}

fn load_monkeys(filename: &str) -> Vec<Monkey> {
    let mut f = File::open(filename).expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    return s.split("\n\n").map(
        |row| { parse_monkey(row) }).collect::<Vec<_>>();
}

fn execute_operation(operation_type: char, operation_arg: &OperationArg, value: i32) -> i32 {
    let arg_value = match operation_arg {
        OperationArg::Constant(x) => *x,
        OperationArg::Old => value
    };

    match operation_type {
        '*' => value * arg_value,
        '+' => value + arg_value,
        _ => panic!("Unknown operation type")
    }
}

fn check_condition(arg: i32, value: i32) -> bool {
    return value % arg == 0;
}

fn execute_round(mut monkeys: Vec<Monkey>, destress: bool) -> Vec<Monkey> {
    for monkey_idx in 0..monkeys.len() {
        let monkey = monkeys.get(monkey_idx).expect("").clone();

       // println!("Monkey {}:", monkey_idx);

        for item in monkey.items.iter() {
       //     println!("Monkey inspects item with worry of {}", item);

            let mut worry = execute_operation(monkey.operation_type, &monkey.operation_arg, *item);

       //     println!("Worry increases to {}", a);

       if (destress) {
            worry = worry / 3;
       }
       //     println!("Worry decreases to {}", b);

            let destination = 
                if check_condition(monkey.test_div_by, worry) {
                    monkey.true_dest 
                } else {
                    monkey.false_dest
                };
            
       //     println!("Item passed to {}", destination);

            let destination_monkey = monkeys.get_mut(destination as usize).expect("Unable to find destination monkey");

            destination_monkey.items.push(worry);
        }
        
        let mut mut_monkey = monkeys.get_mut(monkey_idx).expect("");
        mut_monkey.inspections += mut_monkey.items.len() as i64;
        mut_monkey.items.clear();
    }

    return monkeys;
}

fn execute_rounds(mut monkeys: Vec<Monkey>, count: i32, destress: bool) -> Vec<Monkey> {
    for _ in 0..count {
        monkeys = execute_round(monkeys, destress);

        for (i, monkey) in (0..).zip(monkeys.iter()) {
            println!("Monkey {} has items {}", i, monkey.items.iter().map(|i| i.to_string()).join(","));
        }
    }

    return monkeys;
}

pub fn run_day11() {
    println!("Starting day 11!");
    
    let monkeys = load_monkeys("data/day11.txt");
   
    let results = execute_rounds(monkeys.clone(), 20, true);

    let shenanigans = 
       results.iter().map(|m| m.inspections).sorted_by_key(|i| -i).take(2).collect_vec();

    println!("Part 1 strength: {}", shenanigans.get(0).expect("") * shenanigans.get(1).expect(""));


    let results_2 = execute_rounds(monkeys.clone(), 10000, false);

    let shenanigans_2 = 
       results.iter().map(|m| m.inspections).sorted_by_key(|i| -i).take(2).collect_vec();

    println!("Part 2 strength: {}", shenanigans_2.get(0).expect("") * shenanigans_2.get(1).expect(""));
}

mod tests {
    use itertools::Itertools;

    #[test]
    fn test_monkeys_part_1() {
        let monkeys = super::load_monkeys("data/day11_test.txt");

        let results = super::execute_rounds(monkeys, 20, true);

        let shenanigans = 
            results.iter().map(|m| m.inspections).sorted_by_key(|i| -i).take(2).collect_vec();

        let score = shenanigans.get(0).expect("") * shenanigans.get(1).expect("");

        assert_eq!(score, 10605);
    }

    #[test]
    fn test_monkeys_part_2() {
        let monkeys = super::load_monkeys("data/day11_test.txt");

        let results = super::execute_rounds(monkeys, 10000, false);

        let shenanigans = 
            results.iter().map(|m| m.inspections).sorted_by_key(|i| -i).take(2).collect_vec();

        let score = shenanigans.get(0).expect("") * shenanigans.get(1).expect("");

        assert_eq!(score, 2713310158);
    }
}