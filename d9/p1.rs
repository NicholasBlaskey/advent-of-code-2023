use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let path = Path::new("p1.in");
    //let path = Path::new("t2.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };


    let mut lines: Vec<String> = BufReader::new(file).lines().filter_map(|e| e.ok()).collect();
    lines.insert(0, ".".repeat(lines[0].len()));
    lines.push(".".repeat(lines[0].len()));
    for i in 0..(lines.len()) {
        let new = ".".to_owned() + &lines[i] + ".";
        lines[i] = new;
    }

    let mut found = false;
    let mut i = 0;
    let mut j = 0;
    for l in &lines {
        j = 0;
        for c in l.chars() {
            if c == 'S' {
                found = true;
                break
            }
            j += 1;
        }
        if found {
            break;
        }

        i += 1;
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // TODO unhardcode this first step.
    visited.insert((i, j));
    i -= 1;

    let mut steps = 0;
    loop {
        visited.insert((i, j));
        steps += 1;

        let north = (i-1, j);
        let south = (i+1, j);

        let west = (i, j-1);
        let east = (i, j+1);

        if lines[i].chars().nth(j).unwrap() == '|' {
            (i, j) = pick_between(&lines, &mut visited, north, south);
        } else if lines[i].chars().nth(j).unwrap() == '-' {
            (i, j) = pick_between(&lines, &mut visited, east, west);
        } else if lines[i].chars().nth(j).unwrap() == 'L' {
            (i, j) = pick_between(&lines, &mut visited, north, east);
        } else if lines[i].chars().nth(j).unwrap() == 'J' {
            (i, j) = pick_between(&lines, &mut visited, north, west);
        } else if lines[i].chars().nth(j).unwrap() == '7' {
            (i, j) = pick_between(&lines, &mut visited, south, west);
        } else if lines[i].chars().nth(j).unwrap() == 'F' {
            (i, j) = pick_between(&lines, &mut visited, south, east);
        } else if lines[i].chars().nth(j).unwrap() == '.' {
            println!("BLEH BLEH");
        } else if lines[i].chars().nth(j).unwrap() == 'S' {
            break
        }
    }

    println!("{}", steps / 2);
}

fn pick_between(lines: &Vec<String>, visited: &mut HashSet<(usize, usize)>, a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    if visited.contains(&a) {
        return b;
    } else if visited.contains(&b) {
        return a;
    }

    if lines[a.0].chars().nth(a.1).unwrap() == '.' || lines[a.0].chars().nth(a.1).unwrap() == 'S' {
        return b;
    }
    return a;
}
