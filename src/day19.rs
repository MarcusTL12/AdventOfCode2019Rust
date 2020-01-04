use super::intcode::IntcodeMachine;

pub const PARTS: [fn(); 2] = [part1, part2];

fn pulled(program: &IntcodeMachine, x: usize, y: usize) -> bool {
    let mut nprog = program.clone();
    nprog.input(x as i64);
    nprog.input(y as i64);
    nprog.run();
    match nprog.output() {
        Some(1) => true,
        _ => false,
    }
}

fn part1() {
    let program = IntcodeMachine::from_file("inputfiles/day19.txt");
    //
    let ans: usize = (0..50)
        .map(|i| (0..50).filter(|&j| pulled(&program, j, i)).count())
        .sum();
    //
    println!("Area: {}", ans);
    //
}

fn widestat(program: &IntcodeMachine, n: usize) -> (usize, usize) {
    let y = n;
    let mut x = 0;
    while !pulled(program, x, y) {
        x += 1;
    }
    let mut i = 1;
    while pulled(program, x + i, y - i) {
        i += 1;
    }
    (i, x)
}

fn part2() {
    let program = IntcodeMachine::from_file("inputfiles/day19.txt");
    //
    let mut low = 8;
    let mut high = 16;
    //
    while widestat(&program, high).0 < 100 {
        low = high;
        high <<= 1;
    }
    //
    while low < high {
        let mid = (low + high) / 2;
        if widestat(&program, mid).0 < 100 {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    //
    let x = widestat(&program, low).1;
    println!("{}", x * 10_000 + low - 99);
    //
}
