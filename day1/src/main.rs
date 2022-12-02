use std::time::Instant;

pub fn main() {
    let start = Instant::now();
    let input: Vec<Vec<u32>> = include_str!("input")
        .split("\n\n")
        .map(|x| x.lines().filter_map(|i| str::parse(i).ok()).collect())
        .collect();

    // part 1
    let mut totals: Vec<u32> = input.iter().map(|v| v.iter().sum()).collect();
    totals.sort_unstable();
    let max = totals.last().unwrap();
    // part 2
    let top_3_sum: u32 = totals.iter().rev().take(3).sum();

    let end = Instant::now();
    println!("Time: {:#?}", end - start);

    println!("Part 1: {max}");
    println!("Part 2: {top_3_sum}");
}
