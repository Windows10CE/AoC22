use std::{time::Instant, collections::HashSet};

fn main() {
    let start = Instant::now();

    let input = include_bytes!("input");

    let first_four_different_index = input
        .windows(4)
        .enumerate()
        .find(|chars| chars.1.iter().collect::<HashSet<_>>().len() == 4)
        .unwrap()
        .0 + 4;

    let first_fourteen_different_index = input
        .windows(14)
        .enumerate()
        .find(|chars| chars.1.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        .0 + 14;

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);

    println!("Part 1: {first_four_different_index}");
    println!("Part 2: {first_fourteen_different_index}");
}
