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


    let mut sum = 0;
    for l in lines {
        let mut seq_vec: Vec<i64> = Vec::new();
        for d in l.split(" ") {
            seq_vec.push(d.parse().unwrap());
        }

        let diffs = calc_diffs(seq_vec);
        sum += diffs[0][0];
    }

    println!("{}", sum);
}

fn calc_diffs(seq_vec: Vec<i64>) -> Vec<Vec<i64>> {
    let mut res: Vec<Vec<i64>> = Vec::new();
    res.push(seq_vec.clone());
    let mut seq_index = 0;
    loop {
        let mut next_vec: Vec<i64> = Vec::new();
        for i in 1..(res[seq_index].len()) {
            next_vec.push(res[seq_index][i] - res[seq_index][i-1]);
        }

        res.push(next_vec);
        seq_index += 1;

        let mut all_zeros = true;
        for v in &res[seq_index] {
            if *v != 0 {
                all_zeros = false;
                break;
            }
        }
        if all_zeros {
            break;
        }
    }

    // Add zero.
    res[seq_index].push(0);
    for i in (0..(res.len() - 1)).rev() {
        let new_val = res[i][0] - res[i+1][0];
        res[i].insert(0, new_val);
    }

    return res;
}
