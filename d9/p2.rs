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

    visited.insert((i, j));
    i -= 1; // this is for actual probleme TODO unhardcode this first step.


    //j += 1; // TODO unhardcode this first step.

    let mut steps = 0;
    loop {
        println!("{:?}", (i, j));
        visited.insert((i, j));
        steps += 1;

        let north = (i-1, j);
        let south = (i+1, j);

        let west = (i, j-1);
        let east = (i, j+1);

        println!("char {}", lines[i].chars().nth(j).unwrap());
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

    println!("visited {}", visited.len());


    let mut inside: HashSet<(usize, usize)> = HashSet::new();
    let mut outside: HashSet<(usize, usize)> = HashSet::new();

    for k in 0..(lines.len()) {
        for m in 0..(lines[k].len()) {
            mark_inside(&lines, &visited, &mut inside, &mut outside, (k, m));
        }
    }

    /*
    i = 0;
    for l in &lines {
        j = 0;
        for _ in l.chars() {
            //println!("! {} {}", inside.len(), outside.len());
            mark_inside(&lines, &visited, &mut inside, &mut outside, (i, j));
            j += 1;
        }
        i += 1;
    }
     */

    //println!("{:?}", inside);
    println!("inside {}", inside.len());
    println!("outside {}", outside.len());

    println!("actual length {} {} {} gotten length {}", lines.len() * lines[0].len(),
             lines.len(), lines[0].len(),
             inside.len() + outside.len() + visited.len());

    let mut sum = 0;
    for k in 0..(lines.len()) {
        for m in 0..(lines[k].len()) {
            if visited.contains(&(k, m)) {
                sum += 1;
            }
            if outside.contains(&(k, m)) {
                sum += 1;
            }
            if inside.contains(&(k, m)) {
                sum += 1;
            }
        }
    }
    println!("sum {}", sum);

    i = 0;
    for l in &lines {
        j = 0;
        for c in l.chars() {
            if inside.contains(&(i, j)) {
                print!("{}", "I");
            } else {
                if visited.contains(&(i, j)) {
                    print!("{}", c);
                } else {
                    print!(" ");
                }
            }
            j += 1;
        }
        println!("");
        i += 1;
    }

    let mut count = 0;
    i = 0;
    for l in &lines {
        let mut parity = 0;
        j = 0;
        for c in l.chars() {
            println!("{}", parity);
            if visited.contains(&(i, j)) {
                if (c == '|' || c == 'J' || c == 'L' || c == 'S') {
                    parity += 1;
                }
            } else {
                if parity % 2 != 0 {
                    count += 1;
                }
            }
            j += 1;
        }
        i += 1;
    }

    println!("{}", count);
}

fn mark_inside(lines: &Vec<String>, visited: &HashSet<(usize, usize)>, inside: &mut HashSet<(usize, usize)>, outside: &mut HashSet<(usize, usize)>, a: (usize, usize)) -> bool {
    if inside.contains(&a) {
        return true;
    }
    if outside.contains(&a) {
        return true;
    }
    if visited.contains(&a) {
        return true;
    }

    let mut point_block: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut is_inside = true;

    // 3, 4
    //println!("NEW");
    stack.push(a);
    loop {
        if stack.len() == 0 {
            break;
        }

        // pop
        let (i, j) = stack.pop().unwrap();
        //println!("{} {} {} visiteed {}", i, j, is_inside, visited.contains(&(i, j)));

        // add i, j to point block.
        point_block.insert((i, j));

        // if we are at a block, we are done.
        if visited.contains(&(i, j)) {
            continue;
        }

        // On the outside if we are on a border.
        if i == 0 || i == (lines.len() - 1) {
            is_inside = false;
            continue;
        }
        if j == 0 || j == (lines[0].len() - 1) {
            is_inside = false;
            continue;
        }

        // Check north south east west.
        let north = (i-1, j);
        let south = (i+1, j);
        let west = (i, j-1);
        let east = (i, j+1);

        if !point_block.contains(&north) {
            stack.push(north);
        }
        if !point_block.contains(&south) {
            stack.push(south);
        }
        if !point_block.contains(&east) {
            stack.push(east);
        }
        if !point_block.contains(&west) {
            stack.push(west);
        }

    }


    for p in point_block {
        if visited.contains(&p) {
            continue;
        }

        if is_inside {
            inside.insert(p);
        } else {
            outside.insert(p);
        }
    }


    return false;
}

fn pick_between(lines: &Vec<String>, visited: &mut HashSet<(usize, usize)>, a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    if visited.contains(&a) && visited.contains(&b) {
        println!("tupes {:?} {:?}", a, b);
        println!("CHARS {} {}", lines[a.0].chars().nth(a.1).unwrap(),
                 lines[b.0].chars().nth(b.1).unwrap(),
        );
        
        //panic!("NOT POSSIBLE");
    }
    //println!("VISITED contians {} {}", );
    
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

