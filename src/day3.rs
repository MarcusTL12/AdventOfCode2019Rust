use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn dirmap(dir: char) -> Complex<i32> {
    match dir {
        'R' => Complex::new(1, 0),
        'L' => Complex::new(-1, 0),
        'U' => Complex::new(0, 1),
        'D' => Complex::new(0, -1),
        _ => Complex::new(0, 0),
    }
}

fn parsemove(m: &str) -> (Complex<i32>, i32) {
    let dir = dirmap(m.chars().next().expect("String is fucked!"));
    let dist: i32 = m[1..].parse().expect("Invalid move");
    (dir, dist)
}

fn allpoints(path: &String) -> HashSet<Complex<i32>> {
    let ma = path.split(",").map(|x| parsemove(&x));
    //
    let mut curpos = Complex::new(0, 0);
    let mut points = HashSet::new();
    for (d, l) in ma {
        for _ in 0..l {
            curpos += &d;
            points.insert(curpos.clone());
        }
    }
    points
}

fn allpoints_enumerated(path: &String) -> Vec<Complex<i32>> {
    let ma = path.split(",").map(|x| parsemove(&x));
    //
    let mut curpos = Complex::new(0, 0);
    let mut points = Vec::new();
    for (d, l) in ma {
        for _ in 0..l {
            curpos += &d;
            points.push(curpos.clone());
        }
    }
    points
}

fn part1() {
    let file = File::open("inputfiles/day3/input.txt").expect("File is fucked");
    let paths: Vec<_> = BufReader::new(file)
        .lines()
        .map(|x| x.expect("File read error!"))
        .map(|x| allpoints(&x))
        .collect();
    //
    let ap = &paths[0];
    let bp = &paths[1];

    let m = ap
        .intersection(&bp)
        .min_by(|a, b| {
            (a.re.abs() + a.im.abs()).cmp(&(b.re.abs() + b.im.abs()))
        })
        .expect("What?");
    //
    println!("{}", m.re.abs() + m.im.abs());
}

fn part2() {
    let file = File::open("inputfiles/day3/input.txt").expect("File is fucked");
    let paths: Vec<_> = BufReader::new(file)
        .lines()
        .map(|x| x.expect("File read error!"))
        .map(|x| allpoints_enumerated(&x))
        .collect();
    //
    let ap = &paths[0];
    let bp = &paths[1];

    let alens: HashMap<_, _> =
        ap.iter().enumerate().map(|x| (x.1, x.0 + 1)).collect();

    let blens: HashMap<_, _> =
        bp.iter().enumerate().map(|x| (x.1, x.0 + 1)).collect();
    //

    let m = ap
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .intersection(&(bp.iter().cloned().collect()))
        .map(|x| alens[x] + blens[x])
        .min_by(|a, b| a.cmp(b))
        .expect("What?")
        .clone();
    //
    println!("{}", m);
}
