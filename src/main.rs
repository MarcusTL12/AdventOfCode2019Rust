mod day1;


fn main() -> std::io::Result<()> {
    let mut done = false;
    
    while !done {
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Dafuq");
        
        let choice: Vec<u32> = choice.split_whitespace()
            .map(|x| x.parse().expect("NaNi"))
            .collect();
        
        if choice.len() == 2 {
            match choice[0] {
                0 => done = true,
                1 => match choice[1] {
                    1 => day1::part1()?,
                    2 => day1::part2()?,
                    _ => println!("Not implemented")
                },
                _ => println!("Not implemented")
            }
        } else {
            println!("Only two parameters");
        }
    }
    
    Ok(())
}
