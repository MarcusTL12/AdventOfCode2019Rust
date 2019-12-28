use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


fn traversefrom(
    orbits: &HashMap<String, String>,
    paths: &mut HashMap<&String, i32>,
    k: &String
) -> i32 {
    if k == "COM" {
        return 0;
    }
    
    let x = if paths[k] == 0 {
        if let Some(y) = orbits.get(k) {
            traversefrom(orbits, paths, y) + 1
        } else {
            0
        }
    } else {
        paths[k]
    };
    
    if let Some(y) = paths.get_mut(k) {
        *y = x
    }
    
    x
}


fn path(orbits: &HashMap<String, String>, k: &String) -> Vec<String> {
    if k == "COM" {
        return vec![String::from("COM")];
    }
    
    let mut p = path(orbits, &orbits[k]);
    p.push(k.clone());
    
    p
}


pub fn part1() -> std::io::Result<()> {
    let file = File::open("inputfiles/day6/input.txt")?;
    
    let orbits: HashMap<_, _> = BufReader::new(file)
        .lines()
        .map(|x| {
            let x = x.expect("File read error!");
            let x = x.split(')')
                .collect::<Vec<_>>();
            (String::from(x[1]), String::from(x[0]))
        })
        .collect();
    //
    
    let mut paths: HashMap<_, _> = orbits.iter()
        .map(|(k, _)| (k, 0))
        .collect();
    //
    
    for (v, _) in orbits.iter() {
        traversefrom(&orbits, &mut paths, &v);
    }
    
    let ans: i32 = paths.iter()
        .map(|(_, v)| v)
        .sum();
    //
    
    println!("{}", ans);
    
    Ok(())
}


pub fn part2() -> std::io::Result<()> {
    let file = File::open("inputfiles/day6/input.txt")?;
    
    let orbits: HashMap<_, _> = BufReader::new(file)
        .lines()
        .map(|x| {
            let x = x.expect("File read error!");
            let x = x.split(')')
                .collect::<Vec<_>>();
            (String::from(x[1]), String::from(x[0]))
        })
        .collect();
    //
    
    let mut paths: HashMap<_, _> = orbits.iter()
        .map(|(k, _)| (k, 0))
        .collect();
    //
    
    let yp = path(&orbits, &String::from("YOU"));
    let sp = path(&orbits, &String::from("SAN"));
    
    let common = {
        let mut i = 0;
        while yp[i] == sp[i] {
            i += 1;
        }
        yp[i - 1].clone()
    };
    
    let ans = traversefrom(&orbits, &mut paths, &String::from("YOU")) +
    traversefrom(&orbits, &mut paths, &String::from("SAN")) -
    traversefrom(&orbits, &mut paths, &common) * 2 - 2;
    
    println!("{}", ans);
    
    Ok(())
}
