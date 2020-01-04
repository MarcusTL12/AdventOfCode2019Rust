use std::fs::File;
use std::io::{BufRead, BufReader};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans: u32 = BufReader::new(
        File::open("inputfiles/day1.txt").expect("File is fucked"),
    )
    .lines()
    .map(|x| x.expect("Line is fucked").parse().expect("NaNi"))
    .map(|x: u32| x / 3 - 2)
    .sum();
    println!("Fuel: {}", ans);
}

fn modfuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + modfuel(fuel)
    }
}

fn part2() {
    let ans: i32 = BufReader::new(
        File::open("inputfiles/day1.txt").expect("File is fucked"),
    )
    .lines()
    .map(|x| x.expect("Line is fucked").parse().expect("NaNi"))
    .map(modfuel)
    .sum();
    println!("Fuel: {}", ans);
}
