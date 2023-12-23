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

        println!("k {}", k);
        if k.chars().last().unwrap() == 'A' {
            println!("pushing");
            positions.push(k.to_string());
        }
    }

    let mut count = 0;
    while !positions.clone().into_iter().all(|x| x.chars().last().unwrap() == 'Z') {
        //println!("{:?}", positions);
        //println!("ARRAY");
        
        let turn_left = turn_lefts[count % turn_lefts.len()];
        for i in 0..(positions.len()) {
            if turn_left {
                positions[i] = (m.get(&positions[i]).unwrap().0).clone();
            } else {
                positions[i] = (m.get(&positions[i]).unwrap().1).clone();
            }
        }
        count += 1;
    }

    println!("{}", count);
}
