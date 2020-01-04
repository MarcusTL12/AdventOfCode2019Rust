use std::fs::File;
use std::io::prelude::*;

use std::char;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut inp = String::new();
    File::open("inputfiles/day16.txt")
        .expect("File is fucked")
        .read_to_string(&mut inp)
        .expect("File is fucked!");
    let mut signal: Vec<_> = inp
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();
    let mut signal2 = vec![0; signal.len()];
    let pattern = vec![0, 1, 0, -1];
    for _ in 0..100 {
        for i in 0..signal.len() {
            let mut n = 0;
            for j in 0..signal.len() {
                n += signal[j] * pattern[((j + 1) / (i + 1)) % 4];
            }
            signal2[i] = n.abs() % 10;
        }
        let temp = signal;
        signal = signal2;
        signal2 = temp;
    }

    let ans: String = signal[0..8]
        .iter()
        .map(|x| char::from_digit(*x as u32, 10).unwrap())
        .collect();
    println!("{}", ans);
}

fn part2() {
    let mut inp = String::new();
    File::open("inputfiles/day16.txt")
        .expect("File is fucked")
        .read_to_string(&mut inp)
        .expect("File is fucked!");
    //
    let index: usize = inp[0..7].parse().unwrap();
    let signal: Vec<_> = inp
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();
    //
    let mut actualsignal: Vec<_> = (index..10_000 * signal.len())
        .map(|n| signal[n % signal.len()])
        .collect();
    //
    for _ in 0..100 {
        for i in (0..actualsignal.len() - 1).rev() {
            actualsignal[i] += actualsignal[i + 1];
            actualsignal[i] %= 10;
        }
    }

    let ans: String = actualsignal[0..8]
        .iter()
        .map(|x| char::from_digit(*x as u32, 10).unwrap())
        .collect();
    println!("{}", ans);
}
