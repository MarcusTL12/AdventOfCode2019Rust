use std::fs::File;
use std::io::prelude::*;


pub fn part1() -> std::io::Result<()> {
    let mut s = String::new();
    
    File::open("inputfiles/day8/input.txt")?
        .read_to_string(&mut s)
        .expect("Error reading file!");
    //
    
    let data: Vec<_> = s.chars().collect();
    
    let w = 25;
    let h = 6;
    let l = w * h;
    let layers = data.len() / l;
    
    let minind = (0..layers).map(|x| &data[(l * x)..(l * (x + 1))])
        .map(|x| x.iter().filter(|y| **y == '0').count())
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap().0;
    //
    
    let minlayer = &data[(l * minind)..(l * (minind + 1))];
    
    let ans = minlayer.iter().filter(|y| **y == '1').count() *
        minlayer.iter().filter(|y| **y == '2').count();
    //
    
    println!("{}", ans);
    
    Ok(())
}


pub fn part2() -> std::io::Result<()> {
    let mut s = String::new();
    
    File::open("inputfiles/day8/input.txt")?
        .read_to_string(&mut s)
        .expect("Error reading file!");
    //
    
    let data: Vec<_> = s.chars().collect();
    
    let w = 25;
    let h = 6;
    let l = w * h;
    let layers = data.len() / l;
    
    let flayer = (0..layers)
    .map(|x| data[(l * x)..(l * (x + 1))].iter().collect::<Vec<_>>())
    .fold(
        (0..l).map(|_| &'2').collect::<Vec<_>>(),
        |a, b| a.iter().zip(b.iter()).map(
            |(x, y)| match **x {
                '2' => *y,
                _ => *x
            }
        ).collect::<Vec<_>>()
    );
    //
    
    
    for y in 0..h {
        for x in 0..w {
            print!("{}", match flayer[y * w + x] {
                '0' => ' ',
                '1' => '█',
                _ => '░'
            })
        }
        println!("");
    }
    
    
    Ok(())
}
