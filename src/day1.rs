use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> std::io::Result<()> {
    let file = File::open("inputfiles/day1.txt")?;
    let ans: u32 = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Line is fucked").parse().expect("NaNi"))
        .map(|x: u32| x / 3 - 2)
        .sum();
    println!("Fuel: {}", ans);
    Ok(())
}

fn modfuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + modfuel(fuel)
    }
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("inputfiles/day1.txt")?;
    let ans: i32 = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Line is fucked").parse().expect("NaNi"))
        .map(modfuel)
        .sum();
    println!("Fuel: {}", ans);
    Ok(())
}
