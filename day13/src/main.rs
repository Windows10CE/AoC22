#![feature(array_chunks)]

use core::slice;
use std::{
    cmp::Ordering,
    time::Instant,
};

#[derive(Clone, PartialEq, Eq)]
enum ListItem {
    Integer(i32),
    List(Vec<ListItem>),
}

impl ListItem {
    fn parse(mut str: &str) -> ListItem {
        ListItem::parse_inner(&mut str)
    }

    fn parse_inner(str: &mut &str) -> ListItem {
        if str.starts_with('[') {
            *str = &str[1..];
            let mut items = vec![];
            loop {
                if str.starts_with(']') {
                    break;
                }
                items.push(ListItem::parse_inner(str));
                if str.starts_with(']') {
                    break;
                }
                *str = &str[1..];
            }
            *str = &str[1..];
            ListItem::List(items)
        } else {
            let first_not_digit = str.find(|c: char| !c.is_ascii_digit()).unwrap();
            let ret = ListItem::Integer(str[..first_not_digit].parse().unwrap());
            *str = &str[first_not_digit..];
            ret
        }
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListItem::Integer(x), ListItem::Integer(y)) => x.cmp(y),
            (x @ ListItem::Integer(_), ListItem::List(ys)) => slice::from_ref(x).cmp(&ys[..]),
            (ListItem::List(xs), y @ ListItem::Integer(_)) => xs[..].cmp(slice::from_ref(y)),
            (ListItem::List(xs), ListItem::List(ys)) => xs.cmp(ys),
        }
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let start = Instant::now();

    let input = include_str!("input");

    let mut packets: Vec<ListItem> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(ListItem::parse)
        .collect();

    let sum_ordered: usize = packets
        .array_chunks::<2>()
        .enumerate()
        .filter(|(_, [first, second])| first.cmp(second).is_lt())
        .map(|(index, _)| index + 1)
        .sum();

    let divider_1 = ListItem::parse("[[2]]");
    let divider_2 = ListItem::parse("[[6]]");
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    packets.sort_unstable();

    let key = (packets.binary_search(&divider_1).unwrap() + 1)
        * (packets.binary_search(&divider_2).unwrap() + 1);

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);

    println!("Part 1: {sum_ordered}");
    println!("Part 2: {key}");
}
