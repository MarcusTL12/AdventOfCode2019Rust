use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

extern crate num;
use num::integer::gcd;


fn visiblestroids(
    board: &Vec<Vec<bool>>, xp: usize, yp: usize
) -> HashSet<(i32, i32)> {
    if !board[yp][xp] {
        return HashSet::new();
    }
    
    board.iter()
        .enumerate()
        .map(
            |(i, row)| row.iter()
                .enumerate()
                .filter(move |(j, v)| **v && (*j != xp || i != yp))
                .map(move |(x, _)| (x, i))
        )
        .flat_map(|x| x)
        .map(
            |(x, y)| {
                let dx = x as i32 - xp as i32;
                let dy = yp as i32 - y as i32;
                
                let g = gcd(dx, dy);
                
                (dx / g, dy / g)
            }
        )
        .collect()
}


fn amtvisiblestroids(board: &Vec<Vec<bool>>, xp: usize, yp: usize) -> usize {
    visiblestroids(board, xp, yp).len()
}


fn getangle((x, y): (i32, i32)) -> f32 {
    -(x as f32).atan2(-y as f32)
}


fn getasteroid(
    board: &Vec<Vec<bool>>, mut x: i32, mut y: i32, (dx, dy): (i32, i32)
) -> (usize, usize) {
    let h = board.len() as i32;
    let w = board[0].len() as i32;
    
    x += dx;
    y -= dy;
    
    while 0 <= x && x < w && 0 <= y && y < h && !board[y as usize][x as usize] {
        x += dx;
        y -= dy;
    }
    
    (x as usize, y as usize)
}


pub fn part1() -> std::io::Result<()> {
    let file = File::open("inputfiles/day10/input.txt")?;
    
    let board = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Line is fucked!"))
        .map(|x| x.chars().map(|x| x == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    //
    
    let h = board.len();
    let w = board[0].len();
    
    let vis: Vec<_> = (0..h).map(
            |y| (0..w).map(|x| amtvisiblestroids(&board, x, y))
            .collect::<Vec<_>>()
        )
        .collect();
    //
    
    let v = vis.iter()
        .map(|row| row.iter().max_by(|a, b| a.cmp(b)).unwrap())
        .max_by(|a, b| a.cmp(b))
        .unwrap();
    //
    
    println!("{}", v);
    
    Ok(())
}


pub fn part2() -> std::io::Result<()> {
    let file = File::open("inputfiles/day10/input.txt")?;
    
    let board = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Line is fucked!"))
        .map(|x| x.chars().map(|x| x == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    //
    
    let h = board.len();
    let w = board[0].len();
    
    let vis: Vec<_> = (0..h).map(
            |y| (0..w).map(|x| amtvisiblestroids(&board, x, y))
                .collect::<Vec<_>>()
        )
        .collect();
    //
    
    let (y, (x, _)) = vis.iter().map(
            |row| row.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap()
        )
        .enumerate()
        .max_by(|(_, (_, a)), (_, (_, b))| a.cmp(b))
        .unwrap();
    //
    
    let dirs = visiblestroids(&board, x, y);
    let mut dirs = dirs.iter()
        .collect::<Vec<_>>();
    //
    dirs.sort_by(
        |p1, p2| getangle(**p1).partial_cmp(&getangle(**p2)).unwrap()
    );
    
    
    let (assx, assy) = getasteroid(&board, x as i32, y as i32, *dirs[199]);
    
    println!("{}", assx * 100 + assy);
    
    Ok(())
}