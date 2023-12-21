use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("p1.in");
    //let path = Path::new("t2.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|e| e.ok()).collect();

    let mut times: Vec<i64> = Vec::new();
    for t in lines[0].split("Time:").last().unwrap().trim().split(" ") {
        if t == "" {
            continue;
        }

        for d in t.chars() {
            times.push(d.to_string().parse().unwrap());
        }
    }

    let mut distances: Vec<i64> = Vec::new();
    for t in lines[1].split("Distance:").last().unwrap().trim().split(" ") {
        if t == "" {
            continue;
        }

        for d in t.chars() {
            distances.push(d.to_string().parse().unwrap());
        }
    }

    let mut time = 0;
    let mut base = 1;
    for t in times.into_iter().rev() {
        time += t * base;
        base *= 10;
    }
    println!("t {}", time);

    let mut distance = 0;
    base = 1;
    for t in distances.into_iter().rev() {
        distance += t * base;
        base *= 10;
    }
    println!("d {}", distance);

    let mut num_ways = 0;
    for time_holding_boat in 0..time {
        if (time - time_holding_boat) * time_holding_boat > distance {
            num_ways += 1;
        }
    }

    println!("{}", num_ways);
}
