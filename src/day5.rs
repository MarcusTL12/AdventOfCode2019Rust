use crate::{
    Day, TaskResult,
    intcode::{IntcodeMachine, parse_intcode_program},
};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let (mut machine, s, r) =
        IntcodeMachine::new_with_channel(parse_intcode_program(&input));

    s.send(1).unwrap();

    machine.run().unwrap();

    r.try_iter().last().unwrap().into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
