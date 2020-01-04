use super::intcode::IntcodeMachine;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut machine = IntcodeMachine::from_file("inputfiles/day5.txt");
    machine.input(1);
    machine.run();
    println!("Diagnostic: {}", machine.outputiter().fold(0, |_, a| a));
}

fn part2() {
    let mut machine = IntcodeMachine::from_file("inputfiles/day5.txt");
    //
    machine.input(5);
    machine.run();
    //
    println!("Diagnostic: {}", machine.outputiter().fold(0, |_, a| a));
}
