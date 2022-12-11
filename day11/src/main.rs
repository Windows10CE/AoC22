#![feature(iter_array_chunks)]

use std::{collections::HashSet, time::Instant};

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    success_monkey: usize,
    fail_monkey: usize,
    inspect_count: u64,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Operand),
    Mult(Operand),
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Old,
    Number(u64),
}

fn main() {
    let start = Instant::now();

    let input = include_str!("input");

    let mut monkeys = vec![];

    for monkey_lines in input.lines().filter(|l| !l.is_empty()).array_chunks::<6>() {
        let item_line = monkey_lines[1];
        let operation_line = monkey_lines[2];
        let test_line = monkey_lines[3];
        let success_line = monkey_lines[4];
        let fail_line = monkey_lines[5];

        monkeys.push(Monkey {
            items: item_line[18..]
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect(),
            operation: match operation_line.chars().nth(23).unwrap() {
                '+' => Operation::Add(
                    operation_line[25..]
                        .parse()
                        .map(Operand::Number)
                        .unwrap_or(Operand::Old),
                ),
                '*' => Operation::Mult(
                    operation_line[25..]
                        .parse()
                        .map(Operand::Number)
                        .unwrap_or(Operand::Old),
                ),
                _ => panic!(),
            },
            test_divisor: test_line[21..].parse().unwrap(),
            success_monkey: success_line[29..].parse().unwrap(),
            fail_monkey: fail_line[30..].parse().unwrap(),
            inspect_count: 0,
        })
    }

    let divisor_product = monkeys
        .iter()
        .map(|m| m.test_divisor)
        .collect::<HashSet<_>>()
        .iter()
        .product();

    let top_two_1 = do_monkeys(monkeys.clone(), 20, true, divisor_product);

    let top_two_2 = do_monkeys(monkeys.clone(), 10000, false, divisor_product);

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);

    println!("Part 1: {}", top_two_1.0 * top_two_1.1);
    println!("Part 2: {}", top_two_2.0 * top_two_2.1);
}

fn do_monkeys(
    mut monkeys: Vec<Monkey>,
    round_count: usize,
    divide: bool,
    divisor_product: u64,
) -> (u64, u64) {
    for _ in 0..round_count {
        for monkey_index in 0..monkeys.len() {
            let operation = monkeys[monkey_index].operation;
            for item_index in 0..monkeys[monkey_index].items.len() {
                let old_item = monkeys[monkey_index].items[item_index];
                let mut new_item = match operation {
                    Operation::Add(operand) => match operand {
                        Operand::Old => old_item * 2,
                        Operand::Number(n) => old_item + n,
                    },
                    Operation::Mult(operand) => match operand {
                        Operand::Old => old_item * old_item,
                        Operand::Number(n) => old_item * n,
                    },
                };
                if divide {
                    new_item /= 3;
                } else {
                    new_item %= divisor_product;
                }
                monkeys[monkey_index].inspect_count += 1;
                if new_item % monkeys[monkey_index].test_divisor == 0 {
                    let success_monkey = monkeys[monkey_index].success_monkey;
                    monkeys[success_monkey].items.push(new_item);
                } else {
                    let fail_monkey = monkeys[monkey_index].fail_monkey;
                    monkeys[fail_monkey].items.push(new_item);
                }
            }
            monkeys[monkey_index].items.clear();
        }
    }
    monkeys.iter().fold((0, 0), |best, m| {
        if m.inspect_count > best.0 {
            (m.inspect_count, best.0)
        } else if m.inspect_count > best.1 {
            (best.0, m.inspect_count)
        } else {
            best
        }
    })
}
