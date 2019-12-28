use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;


pub fn part1() -> std::io::Result<()> {
    let file = File::open("inputfiles/day1.txt")?;
    
    let ans: u32 = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Line is fucked")
            .parse()
            .expect("NaNi")
        )
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
    let timer = Instant::now();
    
    let file = File::open("inputfiles/day1.txt")?;
    
    let ans: i32 = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Line is fucked")
            .parse()
            .expect("NaNi")
        )
        .map(modfuel)
        .sum();
    
    println!("{}", timer.elapsed().as_secs_f32());
    
    println!("Fuel: {}", ans);
    
    Ok(())
}
