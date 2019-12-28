use std::time::Instant;

mod day1;
mod day3;
mod day4;
mod day6;


fn main() -> std::io::Result<()> {
    let mut done = false;
    
    while !done {
        println!("Enter day, part:");
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Dafuq");
        
        let choice: Vec<u32> = choice.split_whitespace()
            .map(|x| x.parse().expect("NaNi"))
            .collect();
        
        if choice.len() == 2 {
            let timer = Instant::now();
            match choice[0] {
                0 => done = true,
                1 => match choice[1] {
                    1 => day1::part1()?,
                    2 => day1::part2()?,
                    _ => println!("Not implemented")
                },
                3 => match choice[1] {
                    1 => day3::part1()?,
                    2 => day3::part2()?,
                    _ => println!("Not implemented")
                },
                4 => match choice[1] {
                    1 => day4::part1()?,
                    2 => day4::part2()?,
                    _ => println!("Not implemented")
                },
                6 => match choice[1] {
                    1 => day6::part1()?,
                    2 => day6::part2()?,
                    _ => println!("Not implemented")
                },
                _ => println!("Not implemented")
            }
            print!("Time: {} seconds", timer.elapsed().as_secs_f32());
        } else {
            println!("Only two parameters");
        }
    }
    
    Ok(())
}
