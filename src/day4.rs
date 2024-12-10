use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn digit_iter(n: u32) -> impl Iterator<Item = u8> {
    (0..).scan(n, |n, _| {
        (*n != 0).then_some({
            let d = *n / 10;
            let r = *n % 10;
            *n = d;
            r as u8
        })
    })
}

fn check_cond<I: Iterator<Item = u8>>(it: I) -> bool {
    let mut double = false;

    for [x, y] in it.map_windows(|&[x, y]| [x, y]) {
        if x < y {
            return false;
        }

        double |= x == y;
    }

    double
}

fn part1(input: String) -> TaskResult {
    let (a, b) = input.trim_ascii_end().split_once('-').unwrap();

    let a = a.parse().unwrap();
    let b = b.parse().unwrap();

    (a..=b)
        .filter(|&n| check_cond(digit_iter(n)))
        .count()
        .into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
