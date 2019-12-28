use std::time::Instant;

extern crate digits_iterator;
use digits_iterator::*;


fn isvalidpass(pass: &i32) -> bool {
    let mut repeat = false;
    let mut decrease = false;
    let mut prevdigit = -1;
    
    for d in pass.digits() {
        if d as i32 == prevdigit {
            repeat = true;
        }
        
        if (d as i32) < prevdigit {
            decrease = true;
        }
        
        prevdigit = d as i32;
    }
    
    repeat && !decrease
}


fn isvalidpass2(pass: &i32) -> bool {
    let mut repeat = false;
    let mut amtrepeat = 0;
    let mut decrease = false;
    let mut prevdigit = -1;
    
    for d in pass.digits() {
        if d as i32 == prevdigit {
            amtrepeat += 1;
        } else {
            if amtrepeat == 1 {
                repeat = true;
            }
            amtrepeat = 0;
        }
        
        if (d as i32) < prevdigit {
            decrease = true;
        }
        
        prevdigit = d as i32;
    }
    
    if amtrepeat == 1 {
        repeat = true;
    }
    
    repeat && !decrease
}


pub fn part1() -> std::io::Result<()> {
    let timer = Instant::now();
    
    let c = (359282..820401).filter(isvalidpass).count();
    
    println!("Time: {}", timer.elapsed().as_secs_f32());
    
    println!("{}", c);
    
    Ok(())
}


pub fn part2() -> std::io::Result<()> {
    let timer = Instant::now();
    
    let c = (359282..820401).filter(isvalidpass2).count();
    
    println!("Time: {}", timer.elapsed().as_secs_f32());
    
    println!("{}", c);
    
    Ok(())
}
