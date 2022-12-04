use std::{ops::RangeInclusive, time::Instant};

fn main() {
    let start = Instant::now();

    let input = include_str!("input");

    let range_pairs: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut ranges = l.split(',').map(|r| {
                let sides = r.split_once('-').unwrap();
                sides.0.parse().unwrap()..=sides.1.parse().unwrap()
            });
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect();

    let either_contains_count = range_pairs
        .iter()
        .filter(|ranges| {
            (ranges.0.start() >= ranges.1.start() && ranges.0.end() <= ranges.1.end())
                || (ranges.1.start() >= ranges.0.start() && ranges.1.end() <= ranges.0.end())
        })
        .count();

    let overlaps_count = range_pairs
        .iter()
        .filter(|ranges| {
            (ranges.0.start() <= ranges.1.end() && ranges.0.end() >= ranges.1.end())
                || (ranges.1.start() <= ranges.0.end() && ranges.1.end() >= ranges.0.end())
        })
        .count();

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);

    println!("Part 1: {either_contains_count}");
    println!("Part 2: {overlaps_count}");
}
