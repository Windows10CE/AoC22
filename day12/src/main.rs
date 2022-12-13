#![allow(clippy::char_lit_as_u8)]

use std::{collections::VecDeque, time::Instant};

struct Node {
    elevation: u8,
    adjacent: Vec<usize>,
    is_start: bool,
    is_end: bool,
}

fn main() {
    let start_time = Instant::now();

    let input_rows: Vec<_> = include_bytes!("input")
        .split(|c| *c == '\n' as u8)
        .collect();

    let row_length = input_rows[0].len();

    let mut nodes: Vec<_> = input_rows
        .iter()
        .copied()
        .flatten()
        .copied()
        .map(|mut elevation| {
            let mut is_start = false;
            let mut is_end = false;
            if elevation == 'S' as u8 {
                elevation = 'a' as u8;
                is_start = true;
            } else if elevation == 'E' as u8 {
                elevation = 'z' as u8;
                is_end = true;
            }
            Node {
                elevation: elevation - 'a' as u8,
                adjacent: vec![],
                is_start,
                is_end,
            }
        })
        .collect();

    for node in 0..nodes.len() {
        let elevation = nodes[node].elevation;
        if node % row_length != 0 && nodes[node - 1].elevation <= elevation + 1 {
            nodes[node].adjacent.push(node - 1);
        }
        if node % row_length != row_length - 1 && nodes[node + 1].elevation <= elevation + 1 {
            nodes[node].adjacent.push(node + 1);
        }
        if node >= row_length && nodes[node - row_length].elevation <= elevation + 1 {
            nodes[node].adjacent.push(node - row_length);
        }
        if node < nodes.len() - row_length && nodes[node + row_length].elevation <= elevation + 1 {
            nodes[node].adjacent.push(node + row_length);
        }
        nodes[node].adjacent.sort_unstable();
    }

    let part1_start = nodes
        .iter()
        .enumerate()
        .find_map(|node| if node.1.is_start { Some(node.0) } else { None })
        .unwrap();

    let mut depths = vec![];
    let mut part1_depth = 0;

    for start in nodes
        .iter()
        .enumerate()
        .filter(|n| n.1.elevation == 0)
        .map(|n| n.0)
        .collect::<Vec<_>>()
    {
        if let Some(depth) = breadth_first_search(&nodes, start) {
            depths.push(depth);
            if start == part1_start {
                part1_depth = depth
            }
        }
    }

    let min_depth = depths.into_iter().min().unwrap();

    let end = Instant::now();
    println!("Total time: {:#?}", end - start_time);

    println!("Part 1: {part1_depth}");
    println!("Part 2: {min_depth}");
}

fn breadth_first_search(nodes: &[Node], start: usize) -> Option<i32> {
    let mut visited = vec![false; nodes.len()];
    visited[start] = true;
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    loop {
        let (current, depth) = queue.pop_front()?;
        if nodes[current].is_end {
            return Some(depth);
        }
        for adjacent in nodes[current].adjacent.iter().copied() {
            if visited[adjacent] {
                continue;
            }
            visited[adjacent] = true;
            queue.push_back((adjacent, depth + 1));
        }
    }
}
