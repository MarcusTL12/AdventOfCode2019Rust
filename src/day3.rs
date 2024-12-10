use std::{
    collections::{BTreeMap, BTreeSet},
    iter,
};

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn path_iter(inp: &str) -> impl Iterator<Item = [i32; 2]> {
    inp.as_bytes()
        .split(|&x| x == b',')
        .flat_map(|s| {
            let mut it = s.iter();
            let dir = match *it.next().unwrap() {
                b'R' => [1, 0],
                b'L' => [-1, 0],
                b'U' => [0, 1],
                b'D' => [0, -1],
                _ => panic!(),
            };
            let n = it.fold(0, |acc, &d| 10 * acc + (d - b'0') as usize);

            iter::repeat_n(dir, n)
        })
        .scan([0, 0], |[x, y], [dx, dy]| {
            *x += dx;
            *y += dy;

            Some([*x, *y])
        })
}

fn part1(input: String) -> TaskResult {
    let (path1, path2) = input.trim_ascii_end().split_once('\n').unwrap();

    let visited1: BTreeSet<_> = path_iter(path1).collect();

    path_iter(path2)
        .filter(|pos| visited1.contains(pos))
        .map(|[x, y]| x.abs() + y.abs())
        .min()
        .unwrap()
        .into()
}

fn part2(input: String) -> TaskResult {
    let (path1, path2) = input.trim_ascii_end().split_once('\n').unwrap();

    let visited1: BTreeMap<_, _> = path_iter(path1)
        .enumerate()
        .map(|(i, pos)| (pos, i))
        .collect();

    path_iter(path2)
        .enumerate()
        .filter_map(|(i, pos)| visited1.get(&pos).map(|j| i + j))
        .min()
        .unwrap()
        .into()
}
