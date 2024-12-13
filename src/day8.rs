use std::{io::Write, mem::transmute};

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

const W: usize = 25;
const H: usize = 6;

fn part1(input: String) -> TaskResult {
    let layers: &[[u8; W * H]] =
        input.trim_ascii_end().as_bytes().as_chunks().0;

    let [_, ones, twos] = layers
        .iter()
        .map(|layer| {
            layer.iter().fold([0u32; 3], |mut counts, &x| {
                counts[(x - b'0') as usize] += 1;
                counts
            })
        })
        .min_by_key(|c| c[0])
        .unwrap();

    (ones * twos).into()
}

fn part2(input: String) -> TaskResult {
    let layers: &[[u8; W * H]] =
        input.trim_ascii_end().as_bytes().as_chunks().0;

    let last_layer = layers.iter().fold([2; W * H], |mut canvas, &layer| {
        for (c, x) in canvas.iter_mut().zip(layer) {
            if *c == 2 {
                *c = x - b'0';
            }
        }

        canvas
    });

    let last_layer: [[u8; W]; H] = unsafe { transmute(last_layer) };

    let mut buffer = Vec::with_capacity((W + 1) * H);

    for line in last_layer {
        for x in line {
            write!(&mut buffer, "{}", match x {
                0 => "  ",
                1 => "██",
                _ => panic!(),
            })
            .unwrap();
        }

        writeln!(&mut buffer).unwrap();
    }

    TaskResult::generic(String::from_utf8(buffer).unwrap())
}
