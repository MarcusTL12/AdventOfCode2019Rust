use super::intcode::IntcodeMachine;

pub fn part1() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day21.txt");
    //
    let ins = "NOT T T
AND A T
AND B T
AND C T
NOT T J
AND D J
WALK
";
    //
    program.sendinput(ins.chars().map(|c| c as i64));
    program.run();
    //
    let out: Vec<_> = program.outputiter().collect();
    let ans = out[out.len() - 1];
    let s: String = out
        .iter()
        .take_while(|&&x| x < 255)
        .map(|&x| x as u8 as char)
        .collect();
    println!("{}", s);
    println!("{}", ans);
    //
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day21.txt");
    //
    let ins = "NOT T T
AND A T
AND B T
AND C T
NOT T J
AND D J
NOT J T
NOT T T
AND E T
OR H T
AND T J
RUN
";
    //
    program.sendinput(ins.chars().map(|c| c as i64));
    program.run();
    //
    let out: Vec<_> = program.outputiter().collect();
    let ans = out[out.len() - 1];
    let s: String = out
        .iter()
        .take_while(|&&x| x < 255)
        .map(|&x| x as u8 as char)
        .collect();
    println!("{}", s);
    println!("{}", ans);
    //
    Ok(())
}
