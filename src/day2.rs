use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn run(program: &mut [i64]) {
    for pc in (0..).step_by(4) {
        let opcode = program[pc];

        match opcode {
            1 | 2 => {
                let a = program[pc + 1];
                let b = program[pc + 2];
                let c = program[pc + 3];
                let f = [|x, y| x + y, |x, y| x * y][(opcode - 1) as usize];
                program[c as usize] =
                    f(program[a as usize], program[b as usize]);
            }
            99 => break,
            _ => panic!(),
        }
    }
}

fn part1(input: String) -> TaskResult {
    let mut program: Vec<i64> = input
        .trim_ascii_end()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    program[1] = 12;

    run(&mut program);

    program[0].into()
}

fn part2(input: String) -> TaskResult {
    let orig_program: Vec<i64> = input
        .trim_ascii_end()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut program = orig_program.clone();

    let [noun, verb] = (0..)
        .flat_map(|noun| (0..=noun).map(move |verb| [noun, verb]))
        .find(|&[noun, verb]| {
            program.copy_from_slice(&orig_program);
            program[1] = noun;
            program[2] = verb;

            run(&mut program);

            program[0] == 19690720
        })
        .unwrap();

    (100 * noun + verb).into()
}
