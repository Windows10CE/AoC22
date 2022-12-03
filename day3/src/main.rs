#![feature(iter_array_chunks)]
// fine, all ascii
#![allow(clippy::char_lit_as_u8)]

use std::{collections::HashSet, time::Instant};

fn to_priority(c: u8) -> u32 {
    if c >= 'A' as u8 && c <= 'Z' as u8 {
        return c as u32 - 'A' as u32 + 27;
    } else if c >= 'a' as u8 && c <= 'z' as u8 {
        return c as u32 - 'a' as u32 + 1;
    }
    unreachable!()
}

fn main() {
    let start = Instant::now();

    let input = include_bytes!("input");

    let sacks: Vec<_> = input
        .split(|c| *c == '\n' as u8)
        .filter(|l| !l.is_empty())
        .collect();

    let sum_priorities: u32 = sacks
        .iter()
        .copied()
        .map(|l| {
            let sack = l.split_at(l.len() / 2);
            (
                sack.0.iter().collect::<HashSet<_>>(),
                sack.1.iter().collect::<HashSet<_>>(),
            )
        })
        .map(|sack| to_priority(**(&sack.0 & &sack.1).iter().next().unwrap()))
        .sum();

    let sum_badges: u32 = sacks
        .iter()
        .copied()
        .map(|sack| sack.iter().collect::<HashSet<_>>())
        .array_chunks::<3>()
        .map(|group| {
            to_priority(
                **(&(&group[0] & &group[1]) & &group[2])
                    .iter()
                    .next()
                    .unwrap(),
            )
        })
        .sum();

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);

    println!("Part 1: {sum_priorities}");
    println!("Part 2: {sum_badges}");
}
