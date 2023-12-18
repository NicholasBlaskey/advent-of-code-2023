use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::convert::TryFrom;
use std::collections::HashMap;

fn main() {
    let path = Path::new("p1.in");
    //let path = Path::new("t2.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    let mut sum = 0;
    let mut word_index = 0;
    let mut word_values: Vec<i32> = Vec::new();

    // gear line_num, gear_line col, word_index
    let mut gear_map: HashMap::<(i32, i32, i32), bool> = HashMap::new();

    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|e| e.ok()).collect();
    for (line_num, line) in lines.iter().enumerate() {
        let mut cur_string = String::from("");
        let mut cur_word_valid = false;
        for (char_num, c) in line.chars().enumerate() {
            let v = c.to_digit(10);
            match v {
                Some(_) => {
                    // Check the word logic.
                    cur_string = [cur_string, c.to_string()].join("");
                    cur_word_valid = true;
                    mark_gears(&lines, &mut gear_map, line_num, char_num, word_index);
                },
                None => {
                    // End word logic.
                    if cur_word_valid {
                        word_values.push(cur_string.parse::<i32>().unwrap());
                        word_index += 1;
                        cur_string = "".to_string();
                        cur_word_valid = false;
                    }
                }
            }
        }

        if cur_word_valid {
            word_values.push(cur_string.parse::<i32>().unwrap());
            word_index += 1;
        }
    }

    for (line_num, _) in lines.iter().enumerate() {
        for (char_num, _) in lines.iter().enumerate() {
            let mut count = 0;
            let mut ratio = 1;
            let mut z = 0;
            for i in 0..word_index {
                if gear_map.contains_key(&(i32::try_from(line_num).unwrap(), i32::try_from(char_num).unwrap(), i)) {
                    count += 1;
                    z += word_values.get(usize::try_from(i).unwrap()).unwrap();
                    ratio *= word_values.get(usize::try_from(i).unwrap()).unwrap();
                }
            }


            if count == 2 {
                sum += ratio;
            }
        }
    }

    println!("{}", sum)
}

// digits ignored now.
fn mark_gears(lines: &Vec<String>,
              gear_map: &mut HashMap<(i32, i32, i32), bool>,
              line_num: usize,
              char_num: usize,
              word_index: i32) -> bool
{
    for line_offset in -1..=1 {
        for char_offset in -1..=1 {
            if line_offset == 0 && char_offset == 0 {
                continue;
            }

            if (line_offset == -1 && line_num == 0) || (line_offset == 1 && line_num == lines.len() - 1) {
                continue;
            }
            if (char_offset == -1 && char_num == 0) || (char_offset == 1 && char_num == lines[line_num].len() - 1) {
                continue;
            }

            // TODO this is not ideal.
            let l = usize::try_from(i32::try_from(line_num).unwrap() + line_offset).unwrap();
            let c = usize::try_from(i32::try_from(char_num).unwrap() + char_offset).unwrap();
            let neighbor_char = lines[l].chars().nth(c).unwrap();
            if neighbor_char == '*' {
                gear_map.insert((i32::try_from(l).unwrap(), i32::try_from(c).unwrap(), word_index), true);
            }
        }
    }

    return false;
}
