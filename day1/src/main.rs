use std::time::Instant;

// the collect is to keep parsing time separate from solve time
#[allow(clippy::needless_collect)]

pub fn main() {
    let parse_start = Instant::now();

    let totals: Vec<u32> = include_str!("input")
        .split("\n\n")
        .map(|x| x.lines().filter_map(|i| str::parse::<u32>(i).ok()).sum())
        .collect();

    let actual_start = Instant::now();

    let top_3 = totals.into_iter().fold([0u32; 3], |mut best, item| {
        if item > best[0] {
            best = [item, best[0], best[1]];
        } else if item > best[1] {
            (best[1], best[2]) = (item, best[1]);
        } else if item > best[2] {
            best[2] = item;
        }
        best
    });
    let top_3_sum = top_3[0] + top_3[1] + top_3[2];

    let end = Instant::now();
    println!("Parse time: {:#?}", actual_start - parse_start);
    println!("Solve time: {:#?}", end - actual_start);
    println!("Total time: {:#?}", end - parse_start);

    println!("Part 1: {}", top_3[0]);
    println!("Part 2: {top_3_sum}");
}
