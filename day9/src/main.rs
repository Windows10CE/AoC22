use std::{collections::HashSet, time::Instant};

fn main() {
    let start = Instant::now();

    let input = include_str!("input");

    let count_1 = count_visited(input, &mut [(i32::MAX / 2, i32::MAX / 2); 2]);
    let count_2 = count_visited(input, &mut [(i32::MAX / 2, i32::MAX / 2); 10]);

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);

    println!("Part 1: {count_1}");
    println!("Part 2: {count_2}");
}

fn count_visited(input: &'static str, knots: &mut [(i32, i32)]) -> usize {
    let mut visited = HashSet::new();

    for (command, count) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        let count = count.parse::<u32>().unwrap();
        for _ in 0..count {
            match command {
                "L" => {
                    knots[0].0 -= 1;
                }
                "R" => {
                    knots[0].0 += 1;
                }
                "U" => {
                    knots[0].1 += 1;
                }
                "D" => {
                    knots[0].1 -= 1;
                }
                _ => panic!(),
            }
            for current_index in 1..knots.len() {
                knots[current_index] =
                    compute_new_pos(knots[current_index - 1], knots[current_index]);
            }
            visited.insert(*knots.last().unwrap());
        }
    }

    visited.len()
}

fn compute_new_pos(target: (i32, i32), mut current: (i32, i32)) -> (i32, i32) {
    if target.0.abs_diff(current.0) == 2 || target.1.abs_diff(current.1) == 2 {
        current.0 += (target.0 - current.0).signum();
        current.1 += (target.1 - current.1).signum();
    }
    current
}
