use std::time::Instant;

mod day1;
mod day10;
mod day12;
mod day14;
mod day16;
mod day18;
mod day20;
mod day3;
mod day4;
mod day6;
mod day8;

fn main() -> std::io::Result<()> {
    let mut done = false;
    while !done {
        println!("Enter day, part:");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Dafuq");

        let choice: Vec<u32> = choice
            .split_whitespace()
            .map(|x| x.parse().expect("NaNi"))
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
                6 => match choice[1] {
                    1 => day6::part1()?,
                    2 => day6::part2()?,
                    _ => println!("Not implemented"),
                },
                8 => match choice[1] {
                    1 => day8::part1()?,
                    2 => day8::part2()?,
                    _ => println!("Not implemented"),
                },
                10 => match choice[1] {
                    1 => day10::part1()?,
                    2 => day10::part2()?,
                    _ => println!("Not implemented"),
                },
                12 => match choice[1] {
                    1 => day12::part1()?,
                    2 => day12::part2()?,
                    _ => println!("Not implemented"),
                },
                14 => match choice[1] {
                    1 => day14::part1()?,
                    2 => day14::part2()?,
                    _ => println!("Not implemented"),
                },
                16 => match choice[1] {
                    1 => day16::part1()?,
                    2 => day16::part2()?,
                    _ => println!("Not implemented"),
                },
                18 => match choice[1] {
                    1 => day18::part1()?,
                    2 => day18::part2()?,
                    _ => println!("Not implemented"),
                },
                20 => match choice[1] {
                    1 => day20::part1()?,
                    2 => day20::part2()?,
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
