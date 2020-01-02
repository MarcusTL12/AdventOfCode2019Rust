use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day22;
mod day24;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod intcode;

fn main() -> std::io::Result<()> {
    let mut done = false;
    while !done {
        println!("Enter day, part:");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Dafuq");

        let choice: Vec<u32> = choice
            .split_whitespace()
            .map(|x| match x.parse() {
                Ok(n) => n,
                _ => 30,
            })
            .collect();
        if choice.len() == 2 {
            let timer = Instant::now();
            match choice[0] {
                0 => done = true,
                1 => match choice[1] {
                    1 => day1::part1()?,
                    2 => day1::part2()?,
                    _ => println!("Not implemented"),
                },
                2 => match choice[1] {
                    1 => day2::part1()?,
                    2 => day2::part2()?,
                    _ => println!("Not implemented"),
                },
                3 => match choice[1] {
                    1 => day3::part1()?,
                    2 => day3::part2()?,
                    _ => println!("Not implemented"),
                },
                4 => match choice[1] {
                    1 => day4::part1()?,
                    2 => day4::part2()?,
                    _ => println!("Not implemented"),
                },
                5 => match choice[1] {
                    1 => day5::part1()?,
                    2 => day5::part2()?,
                    _ => println!("Not implemented"),
                },
                6 => match choice[1] {
                    1 => day6::part1()?,
                    2 => day6::part2()?,
                    _ => println!("Not implemented"),
                },
                7 => match choice[1] {
                    1 => day7::part1()?,
                    2 => day7::part2()?,
                    _ => println!("Not implemented"),
                },
                8 => match choice[1] {
                    1 => day8::part1()?,
                    2 => day8::part2()?,
                    _ => println!("Not implemented"),
                },
                9 => match choice[1] {
                    1 => day9::part1()?,
                    2 => day9::part2()?,
                    _ => println!("Not implemented"),
                },
                10 => match choice[1] {
                    1 => day10::part1()?,
                    2 => day10::part2()?,
                    _ => println!("Not implemented"),
                },
                11 => match choice[1] {
                    1 => day11::part1()?,
                    2 => day11::part2()?,
                    _ => println!("Not implemented"),
                },
                12 => match choice[1] {
                    1 => day12::part1()?,
                    2 => day12::part2()?,
                    _ => println!("Not implemented"),
                },
                13 => match choice[1] {
                    1 => day13::part1()?,
                    2 => day13::part2()?,
                    _ => println!("Not implemented"),
                },
                14 => match choice[1] {
                    1 => day14::part1()?,
                    2 => day14::part2()?,
                    _ => println!("Not implemented"),
                },
                15 => match choice[1] {
                    1 => day15::part1()?,
                    2 => day15::part2()?,
                    _ => println!("Not implemented"),
                },
                16 => match choice[1] {
                    1 => day16::part1()?,
                    2 => day16::part2()?,
                    _ => println!("Not implemented"),
                },
                17 => match choice[1] {
                    1 => day17::part1()?,
                    2 => day17::part2()?,
                    _ => println!("Not implemented"),
                },
                18 => match choice[1] {
                    1 => day18::part1()?,
                    2 => day18::part2()?,
                    _ => println!("Not implemented"),
                },
                19 => match choice[1] {
                    1 => day19::part1()?,
                    2 => day19::part2()?,
                    _ => println!("Not implemented"),
                },
                20 => match choice[1] {
                    1 => day20::part1()?,
                    2 => day20::part2()?,
                    _ => println!("Not implemented"),
                },
                22 => match choice[1] {
                    1 => day22::part1()?,
                    2 => day22::part2()?,
                    _ => println!("Not implemented"),
                },
                24 => match choice[1] {
                    1 => day24::part1()?,
                    2 => day24::part2()?,
                    _ => println!("Not implemented"),
                },
                _ => println!("Not implemented"),
            }
            println!("Time: {} seconds", timer.elapsed().as_secs_f32());
        } else {
            println!("Only two parameters");
        }
    }
    Ok(())
}
