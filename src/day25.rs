use super::intcode::IntcodeMachine;

use regex::Regex;

use std::collections::{HashMap, HashSet, VecDeque};

pub const PARTS: [fn(); 2] = [part1, part2];

// Play the game
fn part1() {
    let mut program = IntcodeMachine::from_file("inputfiles/day25.txt");
    //
    program.run();
    let mut s: String = program.outputiter().map(|c| c as u8 as char).collect();
    println!("{}", s);
    //
    while !program.isdone() {
        let mut inp = String::new();
        std::io::stdin()
            .read_line(&mut inp)
            .expect("Console broke!");
        program.sendinput(inp.chars().map(|c| c as i64));
        program.run();
        s = program.outputiter().map(|c| c as u8 as char).collect();
        println!("{}", s);
    }
    //
}

// Autoplays the game
fn part2() {
    let mut program = IntcodeMachine::from_file("inputfiles/day25.txt");
    //
    let reg1 = Regex::new(r"== (.+) ==").expect("Regex is broken!");
    let reg2 = Regex::new(r"Doors here lead:\n((?:- .+\n)+)")
        .expect("Regex is broken!");
    let reg3 =
        Regex::new(r"Items here:\n((?:- .+\n)+)").expect("Regex is broken!");
    //
    let itemblacklist: HashSet<_> = [
        "photons",
        "infinite loop",
        "escape pod",
        "giant electromagnet",
        "molten lava",
    ]
    .iter()
    .map(|&x| String::from(x))
    .collect();
    //
    let directions: Vec<_> = ["north", "east", "south", "west"]
        .iter()
        .map(|&x| String::from(x))
        .collect();
    //
    let invdir: HashMap<_, _> = [
        ("north", "south"),
        ("east", "west"),
        ("south", "north"),
        ("west", "east"),
    ]
    .iter()
    .map(|&x| (String::from(x.0), String::from(x.1)))
    .collect();
    //
    let mut curroom = String::new();
    let mut cdir = String::new();
    //
    let mut maze: HashMap<_, HashMap<_, _>> = HashMap::new();
    //
    let mut path = VecDeque::new();
    let mut pathtosecroom: Option<Vec<_>> = None;
    //
    let mut donemapping = false;
    //
    let mut itembools = Vec::new();
    let mut inventory = Vec::new();
    //
    program.run();
    //
    while !program.isdone() {
        let s: String = program.outputiter().map(|c| c as u8 as char).collect();
        //
        if !donemapping {
            let nroom = reg1
                .captures_iter(&s)
                .map(|cap| String::from(&cap[1]))
                .fold(None, |_, x| Some(x));
            //
            let doors = if let Some(c) = reg2.captures(&s) {
                Some(
                    c[1].split('\n')
                        .filter_map(|l| {
                            if l.len() > 0 {
                                Some(String::from(&l[2..]))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            };
            //
            let items = if let Some(c) = reg3.captures(&s) {
                Some(
                    c[1].split('\n')
                        .filter_map(|l| {
                            if l.len() > 0 {
                                Some(String::from(&l[2..]))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            };
            //
            if let Some(nroom) = nroom {
                if nroom != curroom {
                    if !maze.contains_key(&nroom) {
                        if let Some(doors) = doors {
                            maze.insert(
                                nroom.clone(),
                                doors
                                    .iter()
                                    .map(|door| (door.clone(), String::new()))
                                    .collect(),
                            );
                        }
                        if !cdir.is_empty() {
                            if let Some(x) = maze.get_mut(&nroom) {
                                if let Some(x) = x.get_mut(&invdir[&cdir]) {
                                    *x = curroom.clone();
                                }
                            }
                        }
                        if !curroom.is_empty() {
                            if let Some(x) = maze.get_mut(&curroom) {
                                if let Some(x) = x.get_mut(&cdir) {
                                    *x = nroom.clone();
                                }
                            }
                        }
                    }
                    curroom = nroom;
                }
            }
            //
            if curroom == "Security Checkpoint" {
                pathtosecroom = Some(path.iter().cloned().collect());
                if let Some(x) = maze.get_mut(&curroom) {
                    if let Some(x) = x.get_mut("east") {
                        *x = String::from("Pressure-Sensitive Floor");
                    }
                }
            }
            //
            if let Some(items) = items {
                for item in
                    items.iter().filter(|&item| !itemblacklist.contains(item))
                {
                    inventory.push(String::from(item));
                    program.sendinput(
                        "take ".chars().chain(item.chars()).map(|c| c as i64),
                    );
                    program.input('\n' as i64);
                }
            }
            //
            if let Some(dir) = directions
                .iter()
                .filter_map(|d| {
                    if let Some(x) = maze[&curroom].get(d) {
                        if x.is_empty() {
                            Some(d)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .fold(None, |_, x| Some(x))
            {
                cdir = dir.clone();
                path.push_back(dir.clone());
                program.sendinput(dir.chars().map(|c| c as i64));
                program.input('\n' as i64);
            } else if let Some(dir) = path.pop_back() {
                program.sendinput(invdir[&dir].chars().map(|c| c as i64));
                program.input('\n' as i64);
            } else {
                donemapping = true;
                if let Some(pathtosecroom) = &pathtosecroom {
                    program.sendinput(
                        pathtosecroom
                            .iter()
                            .flat_map(|dir| {
                                dir.chars().chain(vec!['\n'].into_iter())
                            })
                            .map(|c| c as i64),
                    );
                }
                program.sendinput(
                    inventory
                        .iter()
                        .flat_map(|item| {
                            "drop "
                                .chars()
                                .chain(item.chars())
                                .chain(vec!['\n'].into_iter())
                        })
                        .map(|c| c as i64),
                );
                itembools = (0..inventory.len()).map(|_| false).collect();
                program.sendinput("east\n".chars().map(|c| c as i64));
            }
        } else {
            for i in 0..itembools.len() {
                itembools[i] = !itembools[i];
                program.sendinput(
                    if itembools[i] { "take " } else { "drop " }
                        .chars()
                        .chain(inventory[i].chars())
                        .map(|c| c as i64),
                );
                program.input('\n' as i64);
                if itembools[i] {
                    break;
                }
            }
            program.sendinput("east\n".chars().map(|c| c as i64));
        }
        //
        program.run();
    }
    let s: String = program.outputiter().map(|c| c as u8 as char).collect();
    println!("{}", s);
    //
}
