use std::collections::HashSet;
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
        .sum()
        .abs();
    //
    
    println!("Time: {}", timer.elapsed().as_secs_f32());
    
    println!("{}", m);
    
    Ok(())
}
