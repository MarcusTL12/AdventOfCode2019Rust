use super::intcode::IntcodeMachine;

use std::collections::{HashMap, HashSet, VecDeque};

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn mapmaze(
    program: &mut IntcodeMachine,
) -> (HashMap<Complex<i32>, bool>, usize, Option<Complex<i32>>) {
    let mut maze = HashMap::new();
    maze.insert(Complex::new(0, 0), false);
    //
    let mut curpos = Complex::new(0, 0);
    //
    let dirs = [
        Complex::new(0, -1),
        Complex::new(0, 1),
        Complex::new(-1, 0),
        Complex::new(1, 0),
    ];
    //
    let reversepath = [1, 0, 3, 2];
    //
    let mut target = None;
    //
    let mut pathlen = 0;
    //
    let mut path = VecDeque::new();
    //
    while !program.isdone() {
        if let Some(dir) =
            (0..4).find(|&i| !maze.contains_key(&(curpos + dirs[i])))
        {
            program.input(dir as i64 + 1);
            program.run();
            match program.output() {
                Some(0) => {
                    maze.insert(curpos + dirs[dir], true);
                }
                Some(1) => {
                    curpos += dirs[dir];
                    maze.insert(curpos, false);
                    path.push_back(dir);
                }
                Some(2) => {
                    curpos += dirs[dir];
                    maze.insert(curpos, false);
                    path.push_back(dir);
                    pathlen = path.len();
                    target = Some(curpos);
                }
                Some(x) => panic!(
                    "Intcode machine did not give valid output code: {}",
                    x
                ),
                None => panic!("Intcode machine did not give an output!"),
            }
        } else if let Some(dir) = path.pop_back() {
            let backdir = reversepath[dir];
            program.input(backdir as i64 + 1);
            program.run();
            curpos += dirs[backdir];
            assert_eq!(program.output(), Some(1));
        } else {
            break;
        }
    }
    (maze, pathlen, target)
}

fn part1() {
    let mut program = IntcodeMachine::from_file("inputfiles/day15.txt");
    //
    let (_, pathlen, target) = mapmaze(&mut program);
    //
    if target.is_some() {
        println!("Path is {} long", pathlen);
    } else {
        println!("Did not find path!");
    }
    //
}

fn makemaze(
    maze: &HashMap<Complex<i32>, bool>,
) -> (Vec<Vec<bool>>, Complex<i32>) {
    let minx = maze.keys().min_by(|a, b| a.re.cmp(&b.re)).unwrap().re;
    let miny = maze.keys().min_by(|a, b| a.im.cmp(&b.im)).unwrap().im;
    let maxx = maze.keys().max_by(|a, b| a.re.cmp(&b.re)).unwrap().re;
    let maxy = maze.keys().max_by(|a, b| a.im.cmp(&b.im)).unwrap().im;
    //
    (
        (miny..maxy + 1)
            .map(|i| {
                (minx..maxx + 1)
                    .map(|j| match maze.get(&Complex::new(j, i)) {
                        Some(&x) => x,
                        _ => true,
                    })
                    .collect()
            })
            .collect(),
        -Complex::new(minx, miny),
    )
}

fn _rendermaze(maze: &Vec<Vec<bool>>, target: Complex<i32>) {
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            print!(
                "{}",
                if Complex::new(j as i32, i as i32) == target {
                    "░░"
                } else {
                    if maze[i][j] {
                        "██"
                    } else {
                        "  "
                    }
                }
            )
        }
        println!();
    }
}

fn part2() {
    let mut program = IntcodeMachine::from_file("inputfiles/day15.txt");
    //
    let (maze, _, target) = mapmaze(&mut program);
    //
    let (maze, offset) = makemaze(&maze);
    let target = target.unwrap() + offset;
    //
    // _rendermaze(&maze, target);
    //
    let dirs = [
        Complex::new(0, -1),
        Complex::new(0, 1),
        Complex::new(-1, 0),
        Complex::new(1, 0),
    ];
    //
    let isopen = |p: &Complex<i32>| !maze[p.im as usize][p.re as usize];
    //
    let mut visited = HashSet::new();
    visited.insert(target);
    //
    let mut queue = VecDeque::new();
    queue.push_back((target, 0));
    //
    let mut maxlen = 0;
    //
    while let Some((pos, len)) = queue.pop_front() {
        if len > maxlen {
            maxlen = len;
        }
        for d in &dirs {
            let npos = pos + d;
            if !visited.contains(&npos) && isopen(&npos) {
                visited.insert(npos);
                queue.push_back((npos, len + 1));
            }
        }
    }
    //
    println!("Filled after {} minutes.", maxlen);
    //
}
