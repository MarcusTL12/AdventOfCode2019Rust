use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, VecDeque};

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn loadmaze(filename: &str) -> Vec<Vec<char>> {
    BufReader::new(File::open(filename).expect("File if fucked!"))
        .lines()
        .map(|l| l.expect("Line is fucked!"))
        .map(|l| l.chars().collect())
        .collect()
}

fn bfs_resume(
    maze: &Vec<Vec<char>>,
    pos: Complex<i32>,
    keypos: &mut Vec<Complex<i32>>,
    pathdirs: &mut HashMap<Complex<i32>, Complex<i32>>,
) {
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    let dirs: Vec<_> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(|&(re, im)| Complex::new(re, im))
        .collect();
    //
    while let Some(p) = queue.pop_front() {
        for d in dirs.iter() {
            let npos = p + d;
            if !pathdirs.contains_key(&npos) {
                let cell = maze[npos.im as usize][npos.re as usize];
                if cell != '#' && !cell.is_uppercase() {
                    if cell.is_lowercase() {
                        keypos.push(npos);
                    }
                    queue.push_back(npos);
                    pathdirs.insert(npos, -d);
                }
            }
        }
    }
}

fn bfs(
    maze: &Vec<Vec<char>>,
    pos: Complex<i32>,
) -> (Vec<Complex<i32>>, HashMap<Complex<i32>, Complex<i32>>) {
    let mut keypos = Vec::new();
    let mut pathdirs = HashMap::new();
    pathdirs.insert(pos, Complex::new(0, 0));
    bfs_resume(maze, pos, &mut keypos, &mut pathdirs);
    (keypos, pathdirs)
}

fn makepath(
    pathdirs: &HashMap<Complex<i32>, Complex<i32>>,
    mut pos: Complex<i32>,
) -> Vec<Complex<i32>> {
    let mut path = vec![pos];
    while pathdirs[&pos] != Complex::new(0, 0) {
        pos += pathdirs[&pos];
        path.push(pos);
    }
    path
}

fn moveto(
    maze: &Vec<Vec<char>>,
    pathdirs: &HashMap<Complex<i32>, Complex<i32>>,
    pos: Complex<i32>,
) -> (Vec<Vec<char>>, usize, Complex<i32>) {
    let mut maze = maze.clone();
    let path = makepath(pathdirs, pos);
    for npos in path.iter().rev() {
        let cell = maze[npos.im as usize][npos.re as usize];
        if cell.is_lowercase() {
            if let Some((im, Some((re, _)))) = maze
                .iter()
                .map(|row| {
                    row.iter()
                        .enumerate()
                        .find(|(_, x)| **x == cell.to_ascii_uppercase())
                })
                .enumerate()
                .find(|(_, x)| x.is_some())
            {
                maze[im][re] = '.';
            }
            maze[npos.im as usize][npos.re as usize] = '.';
        }
    }
    (maze, path.len() - 1, path[path.len() - 1])
}

fn solve(mazepos: &(Vec<Vec<char>>, Complex<i32>)) -> usize {
    let mut mem = HashMap::new();
    fn rec(
        mem: &mut HashMap<(Vec<Vec<char>>, Complex<i32>), usize>,
        mazepos: &(Vec<Vec<char>>, Complex<i32>),
    ) -> usize {
        if let Some(x) = mem.get(mazepos) {
            *x
        } else {
            let (maze, pos) = mazepos;
            let (keypos, pathdirs) = bfs(maze, *pos);
            let mut bestl = 0;
            for p in keypos {
                let (nmaze, mut l, _) = moveto(maze, &pathdirs, p);
                l += rec(mem, &(nmaze, p));
                if bestl == 0 || l < bestl {
                    bestl = l;
                }
            }
            mem.insert((maze.clone(), *pos), bestl);
            bestl
        }
    }
    rec(&mut mem, mazepos)
}

fn solve2(mazepos: &(Vec<Vec<char>>, Vec<Complex<i32>>)) -> usize {
    let mut mem = HashMap::new();
    fn rec(
        mem: &mut HashMap<(Vec<Vec<char>>, Vec<Complex<i32>>), usize>,
        mazepos: &(Vec<Vec<char>>, Vec<Complex<i32>>),
    ) -> usize {
        if let Some(x) = mem.get(mazepos) {
            *x
        } else {
            let (maze, pos) = mazepos;
            let (mut keypos, mut pathdirs) = bfs(maze, pos[0]);
            for p in pos[1..].iter() {
                bfs_resume(maze, *p, &mut keypos, &mut pathdirs);
                pathdirs.insert(*p, Complex::new(0, 0));
            }
            let mut bestl = 0;
            for p in keypos {
                let (nmaze, mut l, pm) = moveto(maze, &mut pathdirs, p);
                let mut npos = pos.clone();
                if let Some(x) = pos.iter().position(|&x| x == pm) {
                    npos[x] = p;
                }
                l += rec(mem, &(nmaze, npos));
                if bestl == 0 || l < bestl {
                    bestl = l;
                }
            }
            mem.insert((maze.clone(), pos.clone()), bestl);
            bestl
        }
    }
    rec(&mut mem, mazepos)
}

fn part1() {
    let maze = loadmaze("inputfiles/day18/input.txt");
    let pos: Complex<i32> = if let Some((im, Some((re, _)))) = maze
        .iter()
        .map(|row| row.iter().enumerate().find(|(_, x)| **x == '@'))
        .enumerate()
        .find(|(_, x)| x.is_some())
    {
        Complex::new(re as i32, im as i32)
    } else {
        Complex::new(0, 0)
    };
    let ans = solve(&(maze, pos));
    println!("{}", ans);
}

fn part2() {
    let maze = loadmaze("inputfiles/day18/inputpart2.txt");
    let mut pos = Vec::new();
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == '@' {
                pos.push(Complex::new(j as i32, i as i32));
            }
        }
    }
    let ans = solve2(&(maze, pos));
    println!("{}", ans);
}
