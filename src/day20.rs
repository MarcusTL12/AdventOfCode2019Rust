use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, VecDeque};

use num::Complex;

fn loadmaze(filename: &str) -> Vec<Vec<char>> {
    BufReader::new(File::open(filename).expect("File if fucked!"))
        .lines()
        .map(|l| l.expect("Line is fucked!"))
        .map(|l| l.chars().collect())
        .collect()
}

fn _rendermaze(maze: &Vec<Vec<char>>) {
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            print!("{}", maze[i][j]);
        }
        println!("");
    }
}

fn findportals(
    maze: &Vec<Vec<char>>,
) -> (
    HashMap<Complex<i32>, Complex<i32>>,
    Complex<i32>,
    Complex<i32>,
) {
    let h = maze.len();
    let w = maze[0].len();
    let dirs = vec![
        Complex::<i32>::new(1, 0),
        Complex::<i32>::new(-1, 0),
        Complex::<i32>::new(0, 1),
        Complex::<i32>::new(0, -1),
    ];
    let mut portalqueue = HashMap::new();
    let mut portals = HashMap::new();

    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if maze[i][j].is_uppercase() {
                let mut otherletter = maze[i][j];
                let mut portalcoord = Complex::new(0, 0);
                let mut parity = false;
                for (ind, d) in dirs.iter().enumerate() {
                    let npos = Complex::new(j as i32, i as i32) + d;
                    let cell = maze[npos.im as usize][npos.re as usize];
                    if cell.is_uppercase() {
                        otherletter = cell;
                        if ind == 1 || ind == 3 {
                            parity = true;
                        }
                    } else if cell == '.' {
                        portalcoord = npos;
                    }
                }
                let k: String = if parity {
                    vec![otherletter, maze[i][j]]
                } else {
                    vec![maze[i][j], otherletter]
                }
                .iter()
                .collect();
                if portalcoord != Complex::new(0, 0) {
                    if let Some(othercoord) = portalqueue.remove(&k) {
                        portals.insert(portalcoord, othercoord);
                        portals.insert(othercoord, portalcoord);
                    } else {
                        portalqueue.insert(k, portalcoord);
                    }
                }
            }
        }
    }
    (portals, portalqueue["AA"], portalqueue["ZZ"])
}

fn makeportaldirs(
    portals: &HashMap<Complex<i32>, Complex<i32>>,
    w: usize,
    h: usize,
) -> HashMap<Complex<i32>, (Complex<i32>, i32)> {
    portals
        .iter()
        .map(|(&k, &v)| {
            (
                k,
                (
                    v,
                    if k.re > 2
                        && k.re < w as i32 - 4
                        && k.im > 2
                        && k.im < h as i32 - 4
                    {
                        1
                    } else {
                        -1
                    },
                ),
            )
        })
        .collect()
}

pub fn part1() -> std::io::Result<()> {
    let maze = loadmaze("inputfiles/day20/input.txt");
    //
    let (portals, entrance, exit) = findportals(&maze);
    //
    let mut queue = VecDeque::new();
    queue.push_back(entrance);
    //
    let mut pathdict = HashMap::new();
    pathdict.insert(entrance, Complex::new(0, 0));
    //
    let dirs = vec![
        Complex::<i32>::new(1, 0),
        Complex::<i32>::new(-1, 0),
        Complex::<i32>::new(0, 1),
        Complex::<i32>::new(0, -1),
    ];
    //
    while !pathdict.contains_key(&exit) {
        if let Some(node) = queue.pop_front() {
            for d in &dirs {
                let nnode = node + d;
                if !pathdict.contains_key(&nnode) {
                    let val = maze[nnode.im as usize][nnode.re as usize];
                    if val != '#' {
                        if val.is_uppercase() && node != entrance {
                            let otherside = portals[&node];
                            if !pathdict.contains_key(&otherside) {
                                pathdict.insert(otherside, node);
                                queue.push_back(otherside);
                            }
                        } else if val == '.' {
                            pathdict.insert(nnode, node);
                            queue.push_back(nnode);
                        }
                    }
                }
            }
        } else {
            println!("Did not find a way");
            break;
        }
    }
    //
    let path = {
        let mut curpos = exit;
        let mut path = Vec::new();
        while curpos != Complex::new(0, 0) {
            path.push(curpos);
            curpos = pathdict[&curpos];
        }
        path
    };
    //
    // for p in &path {
    //     maze[p.im as usize][p.re as usize] = '█';
    // }
    // _rendermaze(&maze);
    //
    let ans = path.len() - 1;
    println!("Path length: {}", ans);
    //
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let maze = loadmaze("inputfiles/day20/input.txt");
    //
    let h = maze.len();
    let w = maze[0].len();
    //
    let (portals, entrance, exit) = findportals(&maze);
    let portals = makeportaldirs(&portals, w, h);
    //
    let mut queue = VecDeque::new();
    queue.push_back((entrance, 0));
    //
    let mut pathdict = HashMap::new();
    pathdict.insert((entrance, 0), (Complex::new(0, 0), -1));
    //
    let dirs = vec![
        Complex::<i32>::new(1, 0),
        Complex::<i32>::new(-1, 0),
        Complex::<i32>::new(0, 1),
        Complex::<i32>::new(0, -1),
    ];
    //
    // let maxdepth = 2;
    //
    while !pathdict.contains_key(&(exit, 0)) {
        if let Some((node, layer)) = queue.pop_front() {
            for d in &dirs {
                let nnode = node + d;
                if !pathdict.contains_key(&(nnode, layer)) {
                    let val = maze[nnode.im as usize][nnode.re as usize];
                    if val != '#' {
                        if val.is_uppercase() {
                            if let Some(&(otherside, layerdir)) =
                                portals.get(&node)
                            {
                                if 0 <= layer + layerdir
                                // && layer + layerdir <= maxdepth
                                {
                                    let nnodel = (otherside, layer + layerdir);
                                    if !pathdict.contains_key(&nnodel) {
                                        pathdict.insert(nnodel, (node, layer));
                                        queue.push_back(nnodel);
                                    }
                                }
                            }
                        } else if val == '.' {
                            pathdict.insert((nnode, layer), (node, layer));
                            queue.push_back((nnode, layer));
                        }
                    }
                }
            }
        } else {
            println!("Did not find a path!");
            return Ok(());
        }
    }
    //
    let path = {
        let mut curpos = (exit, 0);
        let mut path = Vec::new();
        while curpos != (Complex::new(0, 0), -1) {
            path.push(curpos.0);
            curpos = pathdict[&curpos];
        }
        path
    };
    //
    // for p in &path {
    //     maze[p.im as usize][p.re as usize] = '█';
    // }
    // _rendermaze(&maze);
    //
    let ans = path.len() - 1;
    println!("Path length: {}", ans);
    //
    Ok(())
}
