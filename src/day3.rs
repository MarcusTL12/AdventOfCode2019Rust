use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

extern crate ndarray;
use ndarray::{arr1, Array1};


fn dirmap(dir: char) -> Array1<i32> {
    match dir {
        'R' => arr1(&[ 1,  0]),
        'L' => arr1(&[-1,  0]),
        'U' => arr1(&[ 0,  1]),
        'D' => arr1(&[ 0, -1]),
        _ => arr1(&[0, 0])
    }
}


fn parsemove(m: &str) -> (Array1<i32>, i32) {
    let dir = dirmap(m.chars().next().expect("String is fucked!"));
    
    let dist: i32 = m[1..].parse().expect("Invalid move");
    
    (dir, dist)
}


fn allpoints(path: &String) -> HashSet<Array1<i32>> {
    let ma = path.split(",")
        .map(|x| parsemove(&x));
    //
    
    let mut curpos = arr1(&[0, 0]);
    
    let mut points = HashSet::new();
    
    for (d, l) in ma {
        for _ in 0..l {
            curpos += &d;
            points.insert(curpos.clone());
        }
    }
    
    points
}


fn allpoints_enumerated(path: &String) -> Vec<Array1<i32>> {
    let ma = path.split(",")
        .map(|x| parsemove(&x));
    //
    
    let mut curpos = arr1(&[0, 0]);
    
    let mut points = Vec::<Array1<i32>>::new();
    
    for (d, l) in ma {
        for _ in 0..l {
            curpos += &d;
            points.push(curpos.clone());
        }
    }
    
    points
}


pub fn part1() -> std::io::Result<()> {
    let timer = Instant::now();
    
    let file = File::open("inputfiles/day3/input.txt")?;
    
    let paths = BufReader::new(file)
        .lines()
        .map(|x| x.expect("File read error!"))
        .map(|x| allpoints(&x))
        .collect::<Vec<HashSet<Array1<i32>>>>();
    //
    
    let ap = &paths[0];
    let bp = &paths[1];
    
    let m = ap.intersection(&bp)
        .min_by(|a, b|
            (a[0].abs() + a[1].abs()).cmp(&(b[0].abs() + b[1].abs()))
        )
        .expect("What?")
        .map(|x| x.abs())
        .sum();
    //
    
    println!("Time: {}", timer.elapsed().as_secs_f32());
    
    println!("{}", m);
    
    Ok(())
}


pub fn part2() -> std::io::Result<()> {
    let timer = Instant::now();
    
    let file = File::open("inputfiles/day3/input.txt")?;
    
    let paths = BufReader::new(file)
        .lines()
        .map(|x| x.expect("File read error!"))
        .map(|x| allpoints_enumerated(&x))
        .collect::<Vec<Vec<Array1<i32>>>>();
    //
    
    let ap = &paths[0];
    let bp = &paths[1];
    
    let alens: HashMap<_, _> = ap.iter()
        .enumerate()
        .map(|x| (x.1, x.0 + 1))
        .collect();
    
    let blens: HashMap<_, _> = bp.iter()
        .enumerate()
        .map(|x| (x.1, x.0 + 1))
        .collect();
    //
    
    let m = ap.iter()
        .cloned()
        .collect::<HashSet<_>>()
        .intersection(&(
            bp.iter()
                .cloned()
                .collect()
        ))
        .map(|x| alens[x] + blens[x])
        .min_by(|a, b| a.cmp(b))
        .expect("What?")
        .clone();
    //
    
    
    println!("Time: {}", timer.elapsed().as_secs_f32());
    
    println!("{}", m);
    
    Ok(())
}
