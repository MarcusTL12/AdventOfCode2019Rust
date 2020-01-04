use super::intcode::IntcodeMachine;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut program = IntcodeMachine::from_file("inputfiles/day9.txt");
    program.input(1);
    program.run();
    let ans: Vec<_> = program.outputiter().collect();
    println!("{:?}", ans);
}

fn part2() {
    let mut program = IntcodeMachine::from_file("inputfiles/day9.txt");
    program.input(2);
    program.run();
    let ans: Vec<_> = program.outputiter().collect();
    println!("{:?}", ans);
}
