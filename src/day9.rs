use super::intcode::IntcodeMachine;

pub fn part1() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day9.txt");
    program.input(1);
    program.run();
    let ans: Vec<_> = program.outputiter().collect();
    println!("{:?}", ans);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day9.txt");
    program.input(2);
    program.run();
    let ans: Vec<_> = program.outputiter().collect();
    println!("{:?}", ans);
    Ok(())
}
