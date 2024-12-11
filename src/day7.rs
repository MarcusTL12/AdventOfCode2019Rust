use std::thread;

use arrayvec::ArrayVec;
use crossbeam::channel;

use crate::{
    Day, TaskResult,
    intcode::{IntcodeMachine, parse_intcode_program},
    permiter::permutations,
};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let program = parse_intcode_program(&input);

    let mut max_val = 0;

    for settings in permutations::<5>() {
        let channels = (0..6)
            .map(|_| channel::unbounded())
            .collect::<ArrayVec<_, 6>>()
            .into_inner()
            .unwrap();

        for (&x, (s, _)) in settings.iter().zip(&channels) {
            s.send(x as i64).unwrap();
        }

        let machines = (0..5)
            .map(|i| {
                IntcodeMachine::new(
                    program.clone(),
                    channels[i].1.clone(),
                    channels[i + 1].0.clone(),
                )
            })
            .collect::<ArrayVec<_, 5>>()
            .into_inner()
            .unwrap();

        for mut m in machines {
            thread::spawn(move || m.run().unwrap());
        }

        channels[0].0.send(0).unwrap();

        max_val = channels[5].1.recv().unwrap().max(max_val);
    }

    max_val.into()
}

fn part2(input: String) -> TaskResult {
    let program = parse_intcode_program(&input);

    let (mut m, s, r) = IntcodeMachine::new_with_channel(program);

    s.send(0).unwrap();
    s.send(4).unwrap();
    m.run().unwrap();

    println!("{}", r.recv().unwrap());

    todo!()
}
