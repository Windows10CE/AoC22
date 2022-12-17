use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

#[derive(Debug)]
struct ParsedValve {
    id: usize,
    flow_rate: u32,
    connections: Vec<usize>,
}

#[derive(Debug)]
struct Valve {
    id: usize,
    flow_rate: u32,
    connections: HashMap<usize, u32>,
}

fn main() {
    let start = Instant::now();

    let input = include_str!("input");

    let valves: Vec<_> = input
        .lines()
        .map(|mut line| {
            line = &line[6..];
            let id = usize::from_str_radix(&line[..2], 36).unwrap();
            line = &line[17..];
            let index = line.find(';').unwrap();
            let flow_rate = line[..index].parse().unwrap();
            line = if line.contains("tunnels") {
                &line[index + 25..]
            } else {
                &line[index + 24..]
            };
            let connections = line
                .split(", ")
                .map(|conn| usize::from_str_radix(conn, 36).unwrap())
                .collect();
            ParsedValve {
                id,
                flow_rate,
                connections,
            }
        })
        .collect();

    let mut actual_valves = vec![];

    for i in 0..valves.len() {
        if valves[i].flow_rate == 0 {
            continue;
        }
        let mut new_connections = HashMap::with_capacity(valves[i].connections.len());
        for c in 0..valves[i].connections.len() {
            let conn = valves[i].connections[c];
            get_connections(
                &valves,
                conn,
                &mut new_connections,
                1,
                &mut HashSet::new(),
            );
        }
        actual_valves.push(Valve {
            id: valves[i].id,
            flow_rate: valves[i].flow_rate,
            connections: new_connections,
        })
    }

    let mut distances: Vec<Vec<u32>> = Vec::with_capacity(actual_valves.len());

    for i in 0..actual_valves.len() {
        distances.push(dijkstra(&actual_valves, i));
    }

    // do the funny solve lol lmao

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);
}

fn get_connections(
    valves: &[ParsedValve],
    id: usize,
    new_connections: &mut HashMap<usize, u32>,
    depth: u32,
    visited: &mut HashSet<usize>,
) {
    if visited.contains(&id) {
        return;
    } else {
        visited.insert(id);
    }
    let index = valves.iter().position(|v| v.id == id).unwrap();
    if valves[index].flow_rate == 0 {
        for i in 0..valves[index].connections.len() {
            let conn = valves[index].connections[i];
            get_connections(valves, conn, new_connections, depth + 1, visited);
        }
    } else if let Some(best) = new_connections.get_mut(&id) {
        if depth < *best {
            *best = depth;
        }
    } else {
        new_connections.insert(id, depth);
    }
}

fn dijkstra(valves: &[Valve], start: usize) -> Vec<u32> {
    let mut result = vec![u32::MAX; valves.len()];

    let mut set: Vec<usize> = (0..result.len()).collect();

    result[start] = 0;

    while !set.is_empty() {
        let current = set
            .iter()
            .map(|elem| (*elem, result[*elem]))
            .min_by_key(|(_, dist)| *dist)
            .unwrap()
            .0;

        set.remove(set.iter().position(|item| *item == current).unwrap());

        for (id, index) in valves[current]
            .connections
            .keys()
            .copied()
            .map(|id| (id, valves.iter().position(|v| v.id == id).unwrap()))
            .filter(|(_, index)| set.contains(index))
        {
            let new = result[current] + valves[current].connections[&id];
            if new < result[index] {
                result[index] = new;
            }
        }
    }
    result
}
