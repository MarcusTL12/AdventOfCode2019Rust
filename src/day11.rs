use super::intcode::IntcodeMachine;

use std::collections::HashMap;

use num::Complex;

pub fn part1() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day11.txt");
    //
    let mut pos = Complex::new(0, 0);
    let mut dir = Complex::new(0, 1);
    //
    let mut hull = HashMap::new();
    //
    while !program.isdone() {
        // Look
        program.input(match hull.get(&pos) {
            Some(&x) => x as i64,
            _ => 0,
        });
        program.run();
        //paint
        let color = program.output().unwrap() != 0;
        if let Some(x) = hull.get_mut(&pos) {
            *x = color;
        } else {
            hull.insert(pos, color);
        }
        //turn
        dir *= Complex::new(0, program.output().unwrap() * (-2) + 1);
        //move
        pos += dir;
    }
    //
    let ans = hull.len();
    println!("{}", ans);
    //
    Ok(())
}

fn rendercode(hull: &HashMap<Complex<i64>, bool>) {
    let xmin = hull.keys().min_by(|a, b| a.re.cmp(&b.re)).unwrap().re;
    let ymin = hull.keys().min_by(|a, b| a.im.cmp(&b.im)).unwrap().im;
    let xmax = hull.keys().max_by(|a, b| a.re.cmp(&b.re)).unwrap().re;
    let ymax = hull.keys().max_by(|a, b| a.im.cmp(&b.im)).unwrap().im;
    for y in (ymin..ymax + 1).rev() {
        for x in xmin..xmax + 1 {
            print!(
                "{}",
                if let Some(&v) = hull.get(&Complex::new(x, y)) {
                    if v {
                        '█'
                    } else {
                        ' '
                    }
                } else {
                    ' '
                }
            )
        }
        println!("");
    }
}

pub fn part2() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day11.txt");
    //
    let mut pos = Complex::new(0, 0);
    let mut dir = Complex::new(0, 1);
    //
    let mut hull = HashMap::new();
    hull.insert(pos, true);
    //
    while !program.isdone() {
        // Look
        program.input(match hull.get(&pos) {
            Some(&x) => x as i64,
            _ => 0,
        });
        program.run();
        //paint
        let color = program.output().unwrap() != 0;
        if let Some(x) = hull.get_mut(&pos) {
            *x = color;
        } else {
            hull.insert(pos, color);
        }
        //turn
        dir *= Complex::new(0, program.output().unwrap() * (-2) + 1);
        //move
        pos += dir;
    }
    //
    rendercode(&hull);
    //
    Ok(())
}
