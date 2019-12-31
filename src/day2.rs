use super::intcode::IntcodeMachine;

pub fn part1() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day2/input.txt");
    program.setmem(1, 12);
    program.setmem(2, 2);
    program.run();
    println!("{}", program.getmem(0));
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    fn trycomb(mut prog: IntcodeMachine, (a, b): (i64, i64)) -> i64 {
        prog.setmem(1, a);
        prog.setmem(2, b);
        prog.run();
        prog.getmem(0)
    }
    let program = IntcodeMachine::from_file("inputfiles/day2/input.txt");
    //
    let ans = (0..100)
        .map(|i| (0..100).map(move |j| (i, j)))
        .flat_map(|x| x)
        .filter(|&x| trycomb(program.clone(), x) == 19690720)
        .map(|(a, b)| 100 * a + b)
        .fold(0, |_, a| a);
    //
    println!("{}", ans);
    //
    Ok(())
}
