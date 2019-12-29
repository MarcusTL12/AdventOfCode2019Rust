use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

use num::Integer;

fn loadrecipie(filename: &str) -> HashMap<String, (u64, Vec<(u64, String)>)> {
    BufReader::new(File::open(filename).expect("File is Fucked!"))
        .lines()
        .map(|x| x.expect("Line is fucked!"))
        .map(|l| {
            let sp: Vec<_> = l.split("=>").collect();
            let inp = sp[0];
            let out = sp[1];
            let inp: Vec<(u64, _)> = inp
                .split(',')
                .map(|x| {
                    let sp: Vec<_> = x.split_whitespace().collect();
                    (sp[0].parse().expect("NaNi?"), String::from(sp[1]))
                })
                .collect();
            let sp: Vec<_> = out.split_whitespace().collect();
            let amt: u64 = sp[0].parse().expect("NaNi?");
            let out = String::from(sp[1]);
            (out, (amt, inp))
        })
        .collect()
}

fn makefuel(
    recipie: &HashMap<String, (u64, Vec<(u64, String)>)>,
    amt: u64,
) -> u64 {
    let mut curprod: HashMap<_, u64> =
        recipie.iter().map(|(x, _)| (x, 0)).collect();
    let mut cursurplus: HashMap<_, u64> =
        recipie.iter().map(|(x, _)| (x, 0)).collect();
    //
    let orestring = String::from("ORE");
    curprod.insert(&&orestring, 0);
    cursurplus.insert(&&orestring, 0);

    fn make(
        curprod: &mut HashMap<&String, u64>,
        cursurplus: &mut HashMap<&String, u64>,
        recipie: &HashMap<String, (u64, Vec<(u64, String)>)>,
        orestring: &String,
        chemical: &String,
        amt: u64,
    ) {
        if chemical == "ORE" {
            if let Some(x) = curprod.get_mut(orestring) {
                *x += amt;
            }
        } else {
            let currecipie = &recipie[chemical];
            let amtrecipies = amt.div_ceil(&currecipie.0);
            if let Some(x) = cursurplus.get_mut(chemical) {
                *x += amtrecipies * currecipie.0 - amt;
            }
            if let Some(x) = curprod.get_mut(chemical) {
                *x += amtrecipies * currecipie.0
            }
            for (inpamt, inpchem) in &currecipie.1 {
                let need = inpamt * amtrecipies;
                if need < cursurplus[inpchem] {
                    if let Some(x) = cursurplus.get_mut(inpchem) {
                        *x -= need;
                    }
                } else {
                    let need = need - cursurplus[inpchem];
                    if let Some(x) = cursurplus.get_mut(inpchem) {
                        *x = 0;
                    }
                    make(
                        curprod, cursurplus, recipie, orestring, inpchem, need,
                    );
                }
            }
        }
    }

    make(
        &mut curprod,
        &mut cursurplus,
        recipie,
        &orestring,
        &String::from("FUEL"),
        amt,
    );
    curprod[&&orestring]
}

pub fn part1() -> std::io::Result<()> {
    let recipie = loadrecipie("inputfiles/day14/input.txt");
    let ans = makefuel(&recipie, 1);
    println!("Need {} ore", ans);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let recipie = loadrecipie("inputfiles/day14/input.txt");
    let availableore = 1_000_000_000_000u64;
    let mut low = 1;
    let mut high = 2;
    while makefuel(&recipie, high) < availableore {
        low = high;
        high <<= 2;
    }
    while high - low > 1 {
        let mid = (high + low) / 2;
        let ore = makefuel(&recipie, mid);
        if ore > availableore {
            high = mid;
        } else if ore < availableore {
            low = mid;
        } else {
            low = mid;
            break;
        }
    }
    println!("Can make {} fuel", low);
    Ok(())
}
