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

    let mut sum = 0;
    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|e| e.ok()).collect();
    for line in lines.iter() {
        let mut winning_nums: HashSet::<i32> = HashSet::new();
        for winning_num in line.split(": ").last().unwrap().split(" | ").next().unwrap().split(" ") {
            if winning_num.trim() == "" {
                continue;
            }
            winning_nums.insert(winning_num.parse().unwrap());
        }


        let mut ticket_value = 0;
        for num_str in line.split(": ").last().unwrap().split(" | ").last().unwrap().split(" ") {
            if num_str.trim() == "" {
                continue;
            }

            let num = num_str.parse().unwrap();
            if winning_nums.contains(&num) {
                if ticket_value == 0 {
                    ticket_value = 1;
                } else {
                    ticket_value *= 2;
                }
            }
        }

        sum += ticket_value;
    }

    println!("{}", sum)
}
