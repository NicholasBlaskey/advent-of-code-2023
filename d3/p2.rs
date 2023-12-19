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
    let mut ticket_copies = vec![1; lines.len()];
    for (line_num, line) in lines.iter().enumerate() {
        let cur_ticket_copies = ticket_copies[line_num];
        sum += cur_ticket_copies;
        //println!("ticket number {} cur_ticket_copies {}", line_num, cur_ticket_copies);

        let mut winning_nums: HashSet::<i32> = HashSet::new();
        for winning_num in line.split(": ").last().unwrap().split(" | ").next().unwrap().split(" ") {
            if winning_num.trim() == "" {
                continue;
            }
            winning_nums.insert(winning_num.parse().unwrap());
        }


        let mut number_winners = 0;
        for num_str in line.split(": ").last().unwrap().split(" | ").last().unwrap().split(" ") {
            if num_str.trim() == "" {
                continue;
            }

            let num = num_str.parse().unwrap();
            if winning_nums.contains(&num) {
                number_winners += 1;
            }
        }

        for i in 0..=number_winners {
            ticket_copies[line_num + i] += cur_ticket_copies;
        }
    }

    println!("{}", sum)
}
