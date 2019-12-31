use std::fs::File;
use std::io::{BufRead, BufReader};

use num::integer::lcm;

use regex::Regex;

fn loadinput(filename: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>")
        .expect("Regex is broken!");
    BufReader::new(File::open(filename).expect("File is broken!"))
        .lines()
        .map(|x| x.expect("Line is broken!"))
        .map(|l| {
            let c = re.captures(&l).unwrap();
            (1..4).map(|i| c[i].parse().expect("NaNi?")).collect()
        })
        .collect()
}

fn applygravity(pos: &Vec<Vec<i32>>, vel: &mut Vec<Vec<i32>>) {
    for i in 0..pos.len() {
        for j in (i + 1)..pos.len() {
            for coord in 0..3 {
                let delta = (pos[j][coord] - pos[i][coord]).signum();
                vel[i][coord] += delta;
                vel[j][coord] -= delta;
            }
        }
    }
}

fn movepos(pos: &mut Vec<Vec<i32>>, vel: &Vec<Vec<i32>>) {
    for i in 0..pos.len() {
        for coord in 0..3 {
            pos[i][coord] += vel[i][coord];
        }
    }
}

fn calcenergy(pos: &Vec<Vec<i32>>, vel: &Vec<Vec<i32>>) -> i32 {
    pos.iter()
        .zip(vel.iter())
        .map(|(p, v)| -> i32 {
            p.iter().map(|x| x.abs()).sum::<i32>()
                * v.iter().map(|x| x.abs()).sum::<i32>()
        })
        .sum()
}

pub fn part1() -> std::io::Result<()> {
    let mut pos = loadinput("inputfiles/day12/input.txt");
    let mut vel: Vec<Vec<_>> =
        pos.iter().map(|x| x.iter().map(|_| 0).collect()).collect();
    //
    for _ in 0..1000 {
        applygravity(&pos, &mut vel);
        movepos(&mut pos, &vel);
    }
    let energy = calcenergy(&pos, &vel);
    println!("Energy: {}", energy);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut pos = loadinput("inputfiles/day12/input.txt");
    let mut vel: Vec<Vec<_>> =
        pos.iter().map(|x| x.iter().map(|_| 0).collect()).collect();
    //
    let origpos = pos.clone();
    let origvel = vel.clone();
    let mut periods: Vec<i64> = vec![0, 0, 0];
    let mut time = 0;
    while periods.iter().map(|x| *x == 0).fold(false, |a, b| a || b) {
        applygravity(&pos, &mut vel);
        movepos(&mut pos, &vel);
        time += 1;
        for i in 0..3 {
            if periods[i] == 0 {
                let mut cyc = true;
                for j in 0..pos.len() {
                    if pos[j][i] != origpos[j][i] || vel[j][i] != origvel[j][i]
                    {
                        cyc = false;
                        break;
                    }
                }
                if cyc {
                    periods[i] = time;
                }
            }
        }
    }
    let totper = periods.iter().fold(1, |a, b| lcm(a, *b));
    println!("Period: {}", totper);
    Ok(())
}
