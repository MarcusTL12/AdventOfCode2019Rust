use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|x: u64| x / 3 - 2)
        .sum::<u64>()
        .into()
}

fn rec(x: u64) -> u64 {
    if x <= 6 {
        return 0;
    }

    let fuel = x / 3 - 2;

    fuel + rec(fuel)
}

fn part2(input: String) -> TaskResult {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .map(rec)
        .sum::<u64>()
        .into()
}
