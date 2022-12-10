use std::time::Instant;

enum Instruction {
    Noop,
    AddX(i32),
}

use Instruction::*;

fn main() {
    let start = Instant::now();

    let input = include_str!("input");

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|instruction| {
            let mut split = instruction.split(' ');
            match split.next().unwrap() {
                "noop" => Noop,
                "addx" => AddX(split.next().unwrap().parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect();

    let mut sum = 0;
    let mut reg: i32 = 1;
    let mut instruction_index = 0;
    let mut delay_cycles = match instructions[0] {
        Noop => 1,
        AddX(_) => 2,
    };
    let mut pixels = Vec::with_capacity(240);

    for cycle in 1..=240 {
        pixels.push(reg.abs_diff((cycle - 1) % 40) < 2);
        if cycle % 40 == 20 {
            sum += cycle * reg;
        }
        delay_cycles -= 1;
        if delay_cycles == 0 {
            match instructions[instruction_index] {
                Noop => (),
                AddX(x) => reg += x,
            }
            instruction_index += 1;
            if instruction_index == instructions.len() {
                break;
            }
            delay_cycles = match instructions[instruction_index] {
                Noop => 1,
                AddX(_) => 2,
            };
        }
    }

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);

    println!("Part 1: {sum}");
    println!("Part 2:");
    for line in pixels.chunks(40) {
        for pixel in line {
            print!("{}", if *pixel { '#' } else { '.' });
        }
        println!();
    }
}
