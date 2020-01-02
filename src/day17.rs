use super::intcode::IntcodeMachine;

pub fn part1() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day17.txt");
    program.run();
    //
    let s: String = program.outputiter().map(|x| x as u8 as char).collect();

    let board: Vec<Vec<_>> = s
        .split('\n')
        .take_while(|l| l.len() > 0)
        .map(|l| l.chars().collect())
        .collect();
    //
    let intersects: usize = (1..board.len() - 1)
        .map(|i| -> usize {
            (1..board[0].len() - 1)
                .filter(|&j| {
                    board[i][j] == '#'
                        && board[i - 1][j] == '#'
                        && board[i + 1][j] == '#'
                        && board[i][j - 1] == '#'
                        && board[i][j + 1] == '#'
                })
                .map(|j| i * j)
                .sum()
        })
        .sum();
    //
    println!("Intersects: {}", intersects);
    //
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut program = IntcodeMachine::from_file("inputfiles/day17.txt");
    program.setmem(0, 2);
    //
    let ins = "A,B,A,B,C,C,B,A,B,C
L,4,R,8,L,6,L,10
L,6,R,8,R,10,L,6,L,6
L,4,L,4,L,10
n
";
    //
    program.sendinput(ins.chars().map(|c| c as i64));
    program.run();
    //
    let out: Vec<_> = program.outputiter().collect();
    let ans = out[out.len() - 1];
    let s: String = out.iter().map(|&x| x as u8 as char).collect();
    //
    println!("{}", s);
    println!("{}", ans);
    //
    Ok(())
}
