#![feature(array_windows)]
use std::{collections::HashSet, time::Instant};

fn main() {
    let start = Instant::now();

    let input = include_str!("input");

    let walls: Vec<Vec<(i32, i32)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let parts = coord.split_once(',').unwrap();
                    (parts.0.parse().unwrap(), parts.1.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let (max_x, max_y) = walls
        .iter()
        .flatten()
        .fold((i32::MIN, i32::MIN), |mut current, item| {
            if item.0 > current.0 {
                current.0 = item.0;
            }
            if item.1 > current.1 {
                current.1 = item.1;
            }
            current
        });

    let border_x = max_x + 1;
    let border_y = max_y + 1;

    let mut spots = HashSet::new();

    for [(start_x, start_y), (end_x, end_y)] in walls
        .iter()
        .flat_map(|wall| wall.array_windows::<2>().copied())
    {
        if start_x != end_x {
            if start_x < end_x {
                for x in start_x..=end_x {
                    spots.insert((x, start_y));
                }
            } else {
                for x in end_x..=start_x {
                    spots.insert((x, start_y));
                }
            }
        } else if start_y != end_y {
            if start_y < end_y {
                for y in start_y..=end_y {
                    spots.insert((start_x, y));
                }
            } else {
                for y in end_y..=start_y {
                    spots.insert((start_x, y));
                }
            }
        }
    }

    let sand_start = (500, 0);

    let mut sand_count_1 = 0;
    'outer: loop {
        let mut sand_pos = sand_start;
        'inner: loop {
            if !spots.contains(&(sand_pos.0, sand_pos.1 + 1)) {
                sand_pos.1 += 1;
            } else if !spots.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if !spots.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                spots.insert(sand_pos);
                sand_count_1 += 1;
                break 'inner;
            }
            if sand_pos.0 == 0 || sand_pos.0 == border_x || sand_pos.1 == border_y {
                break 'outer;
            }
        }
    }

    let mut sand_count_2 = sand_count_1;
    let floor = max_y + 2;
    'outer: loop {
        let mut sand_pos = sand_start;
        'inner: loop {
            if sand_pos.1 + 1 != floor && !spots.contains(&(sand_pos.0, sand_pos.1 + 1)) {
                sand_pos.1 += 1;
            } else if sand_pos.1 + 1 != floor && !spots.contains(&(sand_pos.0 - 1, sand_pos.1 + 1))
            {
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if sand_pos.1 + 1 != floor && !spots.contains(&(sand_pos.0 + 1, sand_pos.1 + 1))
            {
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                spots.insert(sand_pos);
                sand_count_2 += 1;
                if sand_pos == sand_start {
                    break 'outer;
                }
                break 'inner;
            }
        }
    }

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);

    println!("Part 1: {sand_count_1}");
    println!("Part 2: {sand_count_2}");
}
