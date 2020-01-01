use super::intcode::IntcodeMachine;

pub fn part1() -> std::io::Result<()> {
    let mut machine = IntcodeMachine::from_file("inputfiles/day5.txt");
    machine.input(1);
    machine.run();
    println!("Diagnostic: {}", machine.outputiter().fold(0, |_, a| a));
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut machine = IntcodeMachine::from_file("inputfiles/day5.txt");
    //
    machine.input(5);
    machine.run();
    //
    println!("Diagnostic: {}", machine.outputiter().fold(0, |_, a| a));
    Ok(())
}
