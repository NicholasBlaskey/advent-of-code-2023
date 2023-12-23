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

    let mut m: HashMap<String, (String, String)> = HashMap::new();
    for l in lines.iter().skip(2) {
        let k = l.split(" = ").next().unwrap();
        let mut left_v = l.split(" = ").last().unwrap().split(", ").next().unwrap().to_string();
        left_v.remove(0);

        let mut right_v = l.split(" = ").last().unwrap().split(", ").last().unwrap().to_string();
        right_v.pop();

        m.insert(k.to_string(), (left_v.clone(), right_v.clone()));
    }

    let mut count = 0;
    let mut value = "AAA".to_string();
    while value != "ZZZ" {
        let turn_left = turn_lefts[count % turn_lefts.len()];
        if turn_left {
            value = (m.get(&value).unwrap().0).clone();
        } else {
            value = (m.get(&value).unwrap().1).clone();
        }
        count += 1;
    }

    println!("{}", count);
}
