use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn digit_iter(n: u32) -> impl Iterator<Item = u8> + Clone {
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

    let a: u32 = a.parse().unwrap();
    let b: u32 = b.parse().unwrap();

    (a..=b)
        .into_par_iter()
        .filter(|&n| check_cond(digit_iter(n)))
        .count()
        .into()
}

fn check_cond2<I: Iterator<Item = u8> + Clone>(it: I) -> bool {
    for [x, y] in it.clone().map_windows(|&[x, y]| [x, y]) {
        if x < y {
            return false;
        }
    }

    [0].into_iter()
        .chain(it.map(|x| x + 1))
        .chain([11])
        .map_windows(|[a, b, c, d]| a != b && b == c && c != d)
        .any(|x| x)
}

fn part2(input: String) -> TaskResult {
    let (a, b) = input.trim_ascii_end().split_once('-').unwrap();

    let a: u32 = a.parse().unwrap();
    let b: u32 = b.parse().unwrap();

    (a..=b)
        .into_par_iter()
        .filter(|&n| check_cond2(digit_iter(n)))
        .count()
        .into()
}
