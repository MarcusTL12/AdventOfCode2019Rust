extern crate digits_iterator;
use digits_iterator::*;

pub const PARTS: [fn(); 2] = [part1, part2];

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

fn part1() {
    let c = (359282..820401).filter(isvalidpass).count();
    println!("{}", c);
}

fn part2() {
    let c = (359282..820401).filter(isvalidpass2).count();
    println!("{}", c);
}
