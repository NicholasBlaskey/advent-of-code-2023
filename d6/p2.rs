use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::cmp;

fn main() {
    //let path = Path::new("p1.in");
    let path = Path::new("t2.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|e| e.ok()).collect();
    let mut hands: Vec<(String, i64)> = Vec::new();
    for l in lines {
        hands.push((l.split(" ").next().unwrap().to_string(), l.split(" ").last().unwrap().parse().unwrap()));
    }

    hands.sort_by(hand_compare);

    let mut sum = 0;
    for (rank, h) in hands.into_iter().enumerate() {
        sum += h.1 * ((rank as i64) + 1);
    }


    println!("{}", sum);
}

fn get_digit_value(h: &(String, i64)) -> i64 {
    let mut sum: i64 = 0;
    let mut base: i64 = 1 * 100 * 100 * 100 * 100 * 100;
    for c in h.0.chars() {
        let mut v: i64 = c.to_digit(10).unwrap_or(0).into();
        if c == 'A' {
            v = 14;
        } else if c == 'K' {
            v = 13;
        } else if c == 'Q' {
            v = 12;
        } else if c == 'J' {
            v = 1;
        } else if c == 'T' {
            v = 10;
        }

        sum += base * v;
        base /= 100;
    }

    return sum;
}

fn get_pairs(h: &(String, i64)) -> Vec<i64> {
    let mut joker_amounts: i64 = 0;

    let mut m: HashMap<String, i64> = HashMap::new();
    for c in h.0.chars() {
        if c == 'J' {
            joker_amounts += 1;
            continue
        }

        let string_c = c.to_string();
        let pair_count = m.get(&string_c).unwrap_or(&0);
        m.insert(string_c, pair_count + 1);
    }

    let mut v: Vec<i64> = Vec::new();
    for (_, pair_amount) in m.clone().into_iter() {
        v.push(pair_amount);
    }
    v.sort_by(|a, b| b.cmp(a));

    if v.len() == 0 {
        v.push(joker_amounts);
    } else {
        v[0] += joker_amounts;
    }
    
    return v;
}

fn hand_compare(a: &(String, i64), b: &(String, i64)) -> Ordering {
    let a_pairs = get_pairs(a);
    let b_pairs = get_pairs(b);

    for i in 0..(cmp::max(a_pairs.len(), b_pairs.len())) {
        if i >= a_pairs.len() {
            return Ordering::Greater;
        }
        if i >= b_pairs.len() {
            return Ordering::Less;
        }

        if a_pairs[i] > b_pairs[i] {
            return Ordering::Greater;
        }
        if a_pairs[i] < b_pairs[i] {
            return Ordering::Less;
        }
    }

    let a_value = get_digit_value(a);
    let b_value = get_digit_value(b);
    if a_value > b_value {
        return Ordering::Greater;
    } else if a_value < b_value {
        return Ordering::Less;
    }

    return Ordering::Equal;
}
