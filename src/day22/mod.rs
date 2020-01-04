use std::fs::File;
use std::io::{BufRead, BufReader};

use num::{Integer, Num};

use regex::Regex;

mod zn;
use zn::Zn;

pub const PARTS: [fn(); 2] = [part1, part2];

fn linpolycompose<T: Copy + Num>(a: (T, T), b: (T, T)) -> (T, T) {
    (a.0 * b.0, a.0 * b.1 + a.1)
}

fn pow<T: Copy + Integer>(a: Zn<T>, mut b: u64) -> Zn<T> {
    let mut acc = Zn::new(T::one(), a.order());
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

fn linpolyrepeat<T: Copy + Integer>(
    (a, b): (Zn<T>, Zn<T>),
    n: u64,
) -> (Zn<T>, Zn<T>) {
    let an = pow(a, n);
    let one = Zn::new(T::one(), a.order());
    (an, b * (one - an) / (one - a))
}

fn makepoly<T: Integer + Copy + std::str::FromStr>(
    filename: &str,
    n: T,
) -> (Zn<T>, Zn<T>) {
    let one = Zn::new(T::one(), n);
    let zero = Zn::new(T::zero(), n);
    //
    let p1 = (-one, -one);
    let p2 = |m: T| (one, one * m);
    let p3 = |m: T| (one / (one * m), zero);
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

fn part1() {
    let n = 10007;
    let p = makepoly("inputfiles/day22/input.txt", n);

    let ans = (1..n + 1)
        .find(|&x| p.0 * x + p.1 == Zn::new(2019, n))
        .unwrap();
    println!("{}", ans);
}

fn part2() {
    let n1 = 119315717514047 as i64;
    let n2 = 101741582076661 as u64;
    //
    let p = makepoly("inputfiles/day22/input.txt", n1);
    //
    let p2 = linpolyrepeat(p, n2);
    //
    let ans = p2.0 * 2020 + p2.1;
    //
    println!("{:?}", ans);
    //
}
