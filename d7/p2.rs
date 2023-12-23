use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let path = Path::new("p1.in");
    //let path = Path::new("t2.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|e| e.ok()).collect();
    let mut turn_lefts: Vec<bool> = Vec::new();
    for c in lines[0].chars() {
        turn_lefts.push(c == 'L');
    }

    let mut positions: Vec<String> = Vec::new();
    let mut m: HashMap<String, (String, String)> = HashMap::new();
    for l in lines.iter().skip(2) {
        let k = l.split(" = ").next().unwrap();
        let mut left_v = l.split(" = ").last().unwrap().split(", ").next().unwrap().to_string();
        left_v.remove(0);

        let mut right_v = l.split(" = ").last().unwrap().split(", ").last().unwrap().to_string();
        right_v.pop();

        m.insert(k.to_string(), (left_v.clone(), right_v.clone()));

        if k.chars().last().unwrap() == 'A' {
            positions.push(k.to_string());
        }
    }

    let mut cycle_lengths: Vec<i64> = Vec::new();

    for p in positions {
        let mut first_done = false;
        let mut cur_pos = p;
        let mut count: usize = 0;
        loop {
            if cur_pos.chars().last().unwrap() == 'Z' {
                if first_done {
                    break
                } else {
                    cycle_lengths.push(count as i64);
                }
                first_done = true;
            }

            let turn_left = turn_lefts[count % turn_lefts.len()];
            if turn_left {
                cur_pos = (m.get(&cur_pos).unwrap().0).clone();
            } else {
                cur_pos = (m.get(&cur_pos).unwrap().1).clone();
            }

            count += 1;
        }
    }


    cycle_lengths.sort();
    loop {
        if cycle_lengths.len() == 1 {
            break;
        }

        let mut times = cycle_lengths[0];
        loop {
            if times % cycle_lengths[1] == 0 {
                break;
            }

            times += cycle_lengths[0];
        }

        // TODO learn vec slicing.
        let mut new_lengths: Vec<i64> = Vec::new();
        for i in 2..(cycle_lengths.len()) {
            new_lengths.push(cycle_lengths[i]);
        }
        new_lengths.push(times);

        cycle_lengths = new_lengths;
        cycle_lengths.sort();
    }
    println!("{}", cycle_lengths[0]);
}
