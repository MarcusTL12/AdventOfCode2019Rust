use super::intcode::IntcodeMachine;

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let program = IntcodeMachine::from_file("inputfiles/day23.txt");
    //
    let amtmachines = 50;
    //
    let mut programs: Vec<_> = (0..amtmachines)
        .map(|i| {
            let mut nprog = program.clone();
            nprog.input(i as i64);
            nprog
        })
        .collect();
    //
    let mut ans = 0;
    //
    for i in (0..amtmachines).cycle() {
        programs[i].input(-1);
        programs[i].run();
        let packets: Vec<_> = programs[i]
            .outputiter()
            .chunks(3)
            .into_iter()
            .map(|mut it| {
                let adr = it.next().unwrap();
                let xy = (it.next().unwrap(), it.next().unwrap());
                (adr as usize, xy)
            })
            .collect();
        //
        for (adr, (x, y)) in packets {
            if adr == 255 {
                ans = y;
            } else {
                programs[adr].input(x);
                programs[adr].input(y);
            }
        }
        if ans != 0 {
            break;
        }
    }
    //
    println!("{}", ans);
    //
}

fn part2() {
    let program = IntcodeMachine::from_file("inputfiles/day23.txt");
    //
    let amtmachines = 50;
    //
    let mut programs: Vec<_> = (0..amtmachines)
        .map(|i| {
            let mut nprog = program.clone();
            nprog.input(i as i64);
            nprog
        })
        .collect();
    //
    let mut nat = (0, 0);
    let mut prevnaty = 0;
    let mut ans = 0;
    let mut idle = false;
    //
    for i in (0..amtmachines).cycle() {
        if i == 0 {
            if idle {
                if nat.1 == prevnaty {
                    ans = prevnaty;
                }
                prevnaty = nat.1;
                programs[i].input(nat.0);
                programs[i].input(nat.1);
            }
            idle = true;
        }
        programs[i].input(-1);
        programs[i].run();
        let packets: Vec<_> = programs[i]
            .outputiter()
            .chunks(3)
            .into_iter()
            .map(|mut it| {
                let adr = it.next().unwrap();
                let xy = (it.next().unwrap(), it.next().unwrap());
                (adr as usize, xy)
            })
            .collect();
        //
        for (adr, (x, y)) in packets {
            idle = false;
            if adr == 255 {
                nat = (x, y);
            } else {
                programs[adr].input(x);
                programs[adr].input(y);
            }
        }
        //
        if ans != 0 {
            break;
        }
    }
    //
    println!("{}", ans);
    //
}
