use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

pub const PARTS: [fn(); 2] = [part1, part2];

fn loadboard(filename: &str) -> Vec<Vec<bool>> {
    BufReader::new(File::open(filename).expect("File is fucked!"))
        .lines()
        .map(|l| l.expect("Line is fucked!"))
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

fn _renderboard(board: &Vec<Vec<bool>>) {
    let h = board.len();
    let w = board[0].len();
    for i in 0..h {
        for j in 0..w {
            print!("{}", if board[i][j] { '#' } else { ' ' })
        }
        println!("")
    }
    println!("-----");
}

fn part1() {
    let mut board = loadboard("inputfiles/day24/input.txt");
    //
    let h = board.len();
    let w = board[0].len();
    //
    let mut buffer: Vec<Vec<_>> =
        (0..h).map(|_| (0..w).map(|_| false).collect()).collect();
    //
    fn alivenext(board: &Vec<Vec<bool>>, i: usize, j: usize) -> bool {
        let h = board.len();
        let w = board[0].len();
        let c = [
            i > 0 && board[i - 1][j],
            i < h - 1 && board[i + 1][j],
            j > 0 && board[i][j - 1],
            j < w - 1 && board[i][j + 1],
        ]
        .iter()
        .filter(|&&x| x)
        .count();
        //
        if board[i][j] {
            c == 1
        } else {
            1 <= c && c <= 2
        }
    }
    //
    let mut seenbefore = HashSet::new();
    seenbefore.insert(board.clone());
    //
    let mut done = false;
    //
    while !done {
        for i in 0..h {
            for j in 0..w {
                buffer[i][j] = alivenext(&board, i, j);
            }
        }
        let temp = buffer;
        buffer = board;
        board = temp;
        //
        done = seenbefore.contains(&board);
        if !done {
            seenbefore.insert(board.clone());
        }
    }
    //
    let mut biod = 0;
    let mut d = 1;
    for i in 0..h {
        for j in 0..w {
            if board[i][j] {
                biod += d;
            }
            d <<= 1;
        }
    }
    //
    println!("Biodiversity: {}", biod);
    //
}

fn part2() {
    let board_mid = loadboard("inputfiles/day24/input.txt");
    let h = board_mid.len();
    let w = board_mid[0].len();
    //
    let depth = 201;
    let mid = depth / 2;
    let mut board: Vec<Vec<Vec<_>>> = (0..depth)
        .map(|_| (0..h).map(|_| (0..w).map(|_| false).collect()).collect())
        .collect();
    //
    let mut buffer = board.clone();
    //
    board[mid] = board_mid;
    //
    fn countadj(
        board: &Vec<Vec<Vec<bool>>>,
        i: usize,
        j: usize,
        k: usize,
    ) -> usize {
        if i != 2 || j != 2 {
            let h = board.len();
            //
            let mut c = 0;
            //
            c += if i == 0 {
                (k > 0 && board[k - 1][1][2]) as usize
            } else if i == 3 && j == 2 {
                if k < h - 1 {
                    (0..5).filter(|&ind| board[k + 1][4][ind]).count()
                } else {
                    0
                }
            } else {
                board[k][i - 1][j] as usize
            };
            //
            c += if i == 4 {
                (k > 0 && board[k - 1][3][2]) as usize
            } else if i == 1 && j == 2 {
                if k < h - 1 {
                    (0..5).filter(|&ind| board[k + 1][0][ind]).count()
                } else {
                    0
                }
            } else {
                board[k][i + 1][j] as usize
            };
            //
            c += if j == 0 {
                (k > 0 && board[k - 1][2][1]) as usize
            } else if i == 2 && j == 3 {
                if k < h - 1 {
                    (0..5).filter(|&ind| board[k + 1][ind][4]).count()
                } else {
                    0
                }
            } else {
                board[k][i][j - 1] as usize
            };
            //
            c += if j == 4 {
                (k > 0 && board[k - 1][2][3]) as usize
            } else if i == 2 && j == 1 {
                if k < h - 1 {
                    (0..5).filter(|&ind| board[k + 1][ind][0]).count()
                } else {
                    0
                }
            } else {
                board[k][i][j + 1] as usize
            };
            //
            c
        } else {
            0
        }
    }
    //
    fn alivenext(
        board: &Vec<Vec<Vec<bool>>>,
        i: usize,
        j: usize,
        k: usize,
    ) -> bool {
        if !(i != 2 || j != 2) {
            false
        } else {
            let c = countadj(board, i, j, k);
            if board[k][i][j] {
                c == 1
            } else {
                1 <= c && c <= 2
            }
        }
    }
    //
    for _ in 0..200 {
        for k in 0..depth {
            for i in 0..h {
                for j in 0..w {
                    buffer[k][i][j] = alivenext(&board, i, j, k);
                }
            }
        }
        let temp = buffer;
        buffer = board;
        board = temp;
    }
    //
    let ans: usize = board
        .iter()
        .map(|layer| -> usize {
            layer
                .iter()
                .map(|row| row.iter().filter(|&&x| x).count())
                .sum()
        })
        .sum();
    //
    println!("Bugs: {}", ans);
    //
}
