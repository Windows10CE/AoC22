// makes it look nicer :(
#![allow(clippy::redundant_clone)]

use std::{collections::VecDeque, time::Instant};

struct Instruction {
    count: usize,
    from_index: usize,
    to_index: usize,
}

fn main() {
    let start = Instant::now();

    let input = include_str!("input");

    let stacks: Vec<VecDeque<char>> =
        input
            .lines()
            .take_while(|l| l.starts_with('['))
            .fold(Vec::new(), |mut vec, line| {
                let items: Vec<Option<char>> = line
                    .chars()
                    .skip(1)
                    .step_by(4)
                    .map(|c| if c.is_alphabetic() { Some(c) } else { None })
                    .collect();

                if vec.is_empty() {
                    for _ in 0..items.len() {
                        vec.push(VecDeque::new());
                    }
                }
                for (index, item) in items.iter().enumerate() {
                    if let Some(c) = item {
                        vec[index].push_front(*c)
                    }
                }
                vec
            });

    let instructions: Vec<Instruction> = input
        .lines()
        .skip_while(|l| !l.starts_with('m'))
        .take_while(|l| !l.is_empty())
        .map(|inst| {
            let from_index = inst.match_indices(" from").next().unwrap().0;
            let to_index = inst.match_indices(" to").next().unwrap().0;
            Instruction {
                count: inst[5..from_index].parse().unwrap(),
                from_index: inst[from_index + 6..to_index].parse::<usize>().unwrap() - 1,
                to_index: inst[to_index + 4..].parse::<usize>().unwrap() - 1,
            }
        })
        .collect();

    let mut part1_stacks = stacks.clone();

    for instruction in instructions.iter() {
        for _ in 0..instruction.count {
            if let Some(c) = part1_stacks[instruction.from_index].pop_back() {
                part1_stacks[instruction.to_index].push_back(c);
            } else {
                break;
            }
        }
    }

    let mut part2_stacks = stacks.clone();

    for instruction in instructions.iter() {
        let mut temp = VecDeque::with_capacity(instruction.count);
        for _ in 0..instruction.count {
            if let Some(c) = part2_stacks[instruction.from_index].pop_back() {
                temp.push_front(c);
            } else {
                break;
            }
        }
        for c in temp {
            part2_stacks[instruction.to_index].push_back(c);
        }
    }

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);

    print!("Part 1: ");
    for stack in part1_stacks {
        print!("{}", stack.back().unwrap());
    }
    println!();

    print!("Part 2: ");
    for stack in part2_stacks {
        print!("{}", stack.back().unwrap());
    }
    println!();
}
