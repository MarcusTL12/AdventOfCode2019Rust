use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn part1() {
    let inputfilename = "inputfiles/day1.txt";
    
    let file = File::open(inputfilename)
        .expect("Could not open file.");
    
    for line in BufReader::new(file).lines() {
        let line = line.expect("Line is fucked!");
        println!("{}", line);
    }
}
