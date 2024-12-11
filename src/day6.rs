use std::collections::HashMap;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn path_iter<'a>(
    map: &'a HashMap<&'a str, &'a str>,
    from: &'a str,
) -> impl Iterator<Item = &'a str> {
    (0..).scan(from, |body, _| {
        if let Some(&new_body) = map.get(body) {
            let old_body = *body;
            *body = new_body;
            Some(old_body)
        } else {
            None
        }
    })
}

fn part1(input: String) -> TaskResult {
    let map: HashMap<_, _> = input
        .lines()
        .map(|l| l.split_once(')').unwrap())
        .map(|(a, b)| (b, a))
        .collect();

    map.keys()
        .map(|&body| path_iter(&map, body).count())
        .sum::<usize>()
        .into()
}

fn part2(input: String) -> TaskResult {
    let map: HashMap<_, _> = input
        .lines()
        .map(|l| l.split_once(')').unwrap())
        .map(|(a, b)| (b, a))
        .collect();

    let path_you: Vec<_> = path_iter(&map, "YOU").collect();
    let path_san: Vec<_> = path_iter(&map, "SAN").collect();

    let shared_path_length = path_you
        .iter()
        .rev()
        .zip(path_san.iter().rev())
        .filter(|(a, b)| a == b)
        .count();

    (path_you.len() + path_san.len() - 2 * shared_path_length - 2).into()
}
