use super::intcode::IntcodeMachine;

pub fn part1() -> std::io::Result<()> {
    let mut machine = IntcodeMachine::from_file("inputfiles/day5.txt");
    machine.sendinput([1].iter());
    machine.run();
    println!("Diagnostic: {}", machine.outputiter().fold(0, |_, a| a));
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut machine = IntcodeMachine::from_file("inputfiles/day5.txt");
    //
    machine.sendinput([5].iter());
    machine.run();
    //
    println!("Diagnostic: {}", machine.outputiter().fold(0, |_, a| a));
    Ok(())
}
