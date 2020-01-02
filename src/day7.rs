use permutator::Permutation;

use super::intcode::IntcodeMachine;

fn singlepass(program: &IntcodeMachine, seq: Vec<i64>) -> i64 {
    seq.iter()
        .map(|&setting| {
            let mut p = program.clone();
            p.input(setting);
            p
        })
        .fold(0, |signal, mut prog| {
            prog.input(signal);
            prog.run();
            match prog.output() {
                Some(x) => x,
                _ => 0,
            }
        })
}

fn multipass(program: &IntcodeMachine, seq: Vec<i64>) -> i64 {
    let mut programs: Vec<_> = seq
        .iter()
        .map(|&setting| {
            let mut p = program.clone();
            p.input(setting);
            p
        })
        .collect();
    //
    (0..seq.len())
        .cycle()
        .scan(0, |signal, i| {
            if programs[i].isdone() {
                None
            } else {
                programs[i].input(*signal);
                programs[i].run();
                if let Some(nsignal) = programs[i].output() {
                    *signal = nsignal;
                    Some(nsignal)
                } else {
                    panic!("Machine {} did not give output!", i)
                }
            }
        })
        .fold(0, |_, x| x)
}

pub fn part1() -> std::io::Result<()> {
    let program = IntcodeMachine::from_file("inputfiles/day7.txt");
    let mut nums = vec![0, 1, 2, 3, 4];
    //
    let ans = nums
        .permutation()
        .map(|v| singlepass(&program, v))
        .max_by(|a, b| a.cmp(b))
        .unwrap();
    //
    println!("Max signal: {}", ans);
    //
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let program = IntcodeMachine::from_file("inputfiles/day7.txt");
    let mut nums = vec![5, 6, 7, 8, 9];
    //
    let ans = nums
        .permutation()
        .map(|v| multipass(&program, v))
        .max_by(|a, b| a.cmp(b))
        .unwrap();
    //
    println!("Max signal: {:?}", ans);
    //
    Ok(())
}
