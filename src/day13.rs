use super::intcode::IntcodeMachine;

use itertools::Itertools;

use std::collections::HashMap;

fn resetlines(l: usize) {
    println!("\x1b[999D\x1b[{}A", l);
}

pub fn part1() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day13.txt");
    //
    program.run();
    //
    let ans = program
        .outputiter()
        .chunks(3)
        .into_iter()
        .map(|it| it.fold(0, |_, v| v))
        .filter(|&x| x == 2)
        .count();
    //
    println!("{}", ans);
    //
    Ok(())
}

fn renderboard(board: &Vec<Vec<i64>>) {
    for i in 0..board.len() {
        for j in 0..board[1].len() {
            print!(
                "{}",
                match board[i][j] {
                    0 => "  ",
                    1 => "██",
                    2 => "░░",
                    3 => "━━",
                    4 => "⚽ ",
                    _ => "  ",
                }
            );
        }
        println!();
    }
}

pub fn part2() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day13.txt");
    program.setmem(0, 2);
    //
    program.run();
    //
    let mut board = {
        let mapmap: HashMap<_, _> = program
            .outputiter()
            .chunks(3)
            .into_iter()
            .map(|it| {
                let v: Vec<_> = it.collect();
                ((v[0], v[1]), v[2])
            })
            .collect();
        //
        let w = mapmap.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
        let h = mapmap.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
        //
        let board: Vec<Vec<_>> = (0..h + 1)
            .map(|i| {
                (0..w + 1)
                    .map(|j| match mapmap.get(&(j, i)) {
                        Some(&v) => v,
                        _ => 0,
                    })
                    .collect()
            })
            .collect();
        //
        board
    };
    //
    println!("Score: 0");
    renderboard(&board);
    //
    let mut paddle = board
        .iter()
        .map(|row| row.iter().position(|&x| x == 3))
        .find(|x| x.is_some())
        .unwrap()
        .unwrap() as i64;
    let mut ball: i64 = board
        .iter()
        .map(|row| row.iter().position(|&x| x == 4))
        .find(|x| x.is_some())
        .unwrap()
        .unwrap() as i64;
    let mut balldir = 1;
    let h = board.len();
    let mut score = 0;
    //
    while !program.isdone() {
        let inp = if ball + balldir > paddle + 1 {
            1
        } else if ball + balldir < paddle - 1 {
            -1
        } else {
            0
        };
        //
        paddle += inp;
        //
        program.input(inp);
        program.run();
        //
        for it in program.outputiter().chunks(3).into_iter() {
            let v: Vec<_> = it.collect();
            if v[0] == -1 {
                score = v[2];
            } else {
                board[v[1] as usize][v[0] as usize] = v[2];
                if v[2] == 4 {
                    balldir = (v[0] - ball).signum();
                    ball = v[0];
                }
            }
        }
        //
        std::thread::sleep(std::time::Duration::from_millis(10));
        resetlines(h + 2);
        println!("Score: {}                          ", score);
        renderboard(&board);
    }
    //
    println!("Final score: {}", score);
    //
    Ok(())
}
