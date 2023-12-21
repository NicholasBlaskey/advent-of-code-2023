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

    let mut times: Vec<i32> = Vec::new();
    for t in lines[0].split("Time:").last().unwrap().trim().split(" ") {
        if t == "" {
            continue;
        }
        times.push(t.parse().unwrap());
    }

    let mut distances: Vec<i32> = Vec::new();
    for t in lines[1].split("Distance:").last().unwrap().trim().split(" ") {
        if t == "" {
            continue;
        }
        distances.push(t.parse().unwrap());
    }

    let mut mult_num_ways = 1;
    for i in 0..(times.len()) {
        let mut num_ways = 0;
        for time_holding_boat in 0..(times[i]) {
            if (times[i] - time_holding_boat) * time_holding_boat > distances[i] {
                num_ways += 1;
            }
        }

        mult_num_ways *= num_ways;
    }


    println!("{}", mult_num_ways);
}
