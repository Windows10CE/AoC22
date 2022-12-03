// ascii
#![allow(clippy::char_lit_as_u8)]
// clarity
#![allow(clippy::identity_op)]

use std::time::Instant;

static PART1: [[u32; 3]; 3] = [
    [3 + 1, 6 + 2, 0 + 3],
    [0 + 1, 3 + 2, 6 + 3],
    [6 + 1, 0 + 2, 3 + 3],
];

static PART2: [[u32; 3]; 3] = [
    [0 + 3, 3 + 1, 6 + 2],
    [0 + 1, 3 + 2, 6 + 3],
    [0 + 2, 3 + 3, 6 + 1],
];

fn main() {
    let start = Instant::now();

    let input: Vec<(u8, u8)> = include_bytes!("input")
        .split(|c| *c == '\n' as u8)
        .filter(|l| !l.is_empty())
        .map(|l| (l[0] - 'A' as u8, l[2] - 'X' as u8))
        .collect();

    let total_1: u32 = input
        .iter()
        .map(|round| PART1[round.0 as usize][round.1 as usize])
        .sum();

    let total_2: u32 = input
        .iter()
        .map(|round| PART2[round.0 as usize][round.1 as usize])
        .sum();

    let end = Instant::now();
    println!("Time: {:#?}", end - start);

    println!("Part 1: {total_1}");
    println!("Part 1: {total_2}");
}
