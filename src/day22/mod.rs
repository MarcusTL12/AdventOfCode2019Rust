use std::fs::File;
use std::io::{BufRead, BufReader};

use num::{Integer, Num, One, Zero};

use regex::Regex;

mod zn;
use zn::Zn;

pub const PARTS: [fn(); 2] = [part1, part2];

fn linpolycompose<T: Copy + Num>(a: (T, T), b: (T, T)) -> (T, T) {
    (a.0 * b.0, a.0 * b.1 + a.1)
}

fn pow<const N: u64>(a: Zn<N>, mut b: u64) -> Zn<N> {
    let mut acc = Zn::one();
    let mut cur = a;
    
    while b != 0 {
        let (q, r) = b.div_rem(&2);
        b = q;
        if r != 0 {
            acc *= cur;
        }
        cur *= cur;
    }
    
    acc
}

fn linpolyrepeat<const N: u64>(
    (a, b): (Zn<N>, Zn<N>),
    n: u64,
) -> (Zn<N>, Zn<N>) {
    let an = pow(a, n);
    let one = Zn::one();
    (an, b * (one - an) / (one - a))
}

fn makepoly<const N: u64>(
    filename: &str
) -> (Zn<N>, Zn<N>) {
    let one = Zn::new(1);
    let zero = Zn::new(0);
    //
    let p1 = (-one, -one);
    let p2 = |m: i64| (one, one * m);
    let p3 = |m: i64| (one / (one * m), zero);
    //
    let reg0 = String::from("deal into new stack");
    let reg1 = Regex::new(r"cut (-?\d+)").expect("Regex 1 is broken!");
    let reg2 =
        Regex::new(r"deal with increment (\d+)").expect("Regex 2 is broken!");
    //
    BufReader::new(File::open(filename).expect("File is fucked!"))
        .lines()
        .map(|l| l.expect("Line is fucked"))
        .map(|l| {
            if l == reg0 {
                p1
            } else if let Some(c) = reg1.captures(&l) {
                match c[1].parse() {
                    Ok(m) => p2(m),
                    _ => panic!("NaNi?\n{:?}", l),
                }
            } else if let Some(c) = reg2.captures(&l) {
                match c[1].parse() {
                    Ok(m) => p3(m),
                    _ => panic!("NaNi?\n{:?}", l),
                }
            } else {
                panic!("Unexpected line: \n {}", l);
            }
        })
        .fold((one, zero), linpolycompose)
}

fn _makepoly<const N: u64>(
    filename: &str
) -> (Zn<N>, Zn<N>) {
    let one = Zn::one();
    let zero = Zn::zero();
    //
    let p1 = (-one, -one);
    let p2 = |m: i64| (one, one * m);
    let p3 = |m: i64| (one / (one * m), zero);
    //
    let reg0 = String::from("deal into new stack");
    let reg1 = Regex::new(r"cut (-?\d+)").expect("Regex 1 is broken!");
    let reg2 =
        Regex::new(r"deal with increment (\d+)").expect("Regex 2 is broken!");
    //
    BufReader::new(File::open(filename).expect("File is fucked!"))
        .lines()
        .map(|l| l.expect("Line is fucked"))
        .map(|l| {
            println!("{}", l);
            if l == reg0 {
                println!("{:?}", p1);
                p1
            } else if let Some(c) = reg1.captures(&l) {
                let temp = match c[1].parse() {
                    Ok(m) => p2(m),
                    _ => panic!("NaNi?\n{:?}", l),
                };
                println!("{:?}", temp);
                temp
            } else if let Some(c) = reg2.captures(&l) {
                let temp = match c[1].parse() {
                    Ok(m) => p3(m),
                    _ => panic!("NaNi?\n{:?}", l),
                };
                println!("{:?}", temp);
                temp
            } else {
                panic!("Unexpected line: \n {}", l);
            }
        })
        .fold((one, zero), linpolycompose)
}

fn part1() {
    const N: u64 = 10;
    let p = _makepoly::<N>("inputfiles/day22/example.txt");
    
    // let tofind: Zn<N> = Zn::new(2019);
    
    // let ans = p.0 * 4284 + p.1;
    
    // let ans = (1..N + 1)
    //     .find(|&x| p.0 * x + p.1 == tofind);
        // .unwrap();
    println!("{:?}", p);
}

fn part2() {
    const N1: u64 = 119315717514047;
    const N2: u64 = 101741582076661;
    //
    let p = makepoly::<N1>("inputfiles/day22/input.txt");
    //
    let p2 = linpolyrepeat(p, N2);
    //
    let ans = p2.0 * 2020 + p2.1;
    //
    println!("{:?}", ans);
    //
}
