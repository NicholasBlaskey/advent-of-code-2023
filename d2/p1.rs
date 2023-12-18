use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::convert::TryFrom;

fn main() {
    let path = Path::new("p1.in");
    //let path = Path::new("t2.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    let mut sum = 0;

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
                    //cur_string += c;
                    if !cur_word_valid && is_adj_to_non_dot(&lines, line_num, char_num) {
                        cur_word_valid = true;
                    }
                },
                None => {
                    // End word logic.
                    if cur_word_valid {
                        sum += cur_string.parse::<i32>().unwrap();
                    }

                    cur_string = "".to_string();
                    cur_word_valid = false;
                }
            }
        }

        if cur_word_valid {
            sum += cur_string.parse::<i32>().unwrap();
        }
    }

    println!("{}", sum)
}

// digits ignored now.
fn is_adj_to_non_dot(lines: &Vec<String>, line_num: usize, char_num: usize) -> bool {
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
            if neighbor_char != '.' && !neighbor_char.is_digit(10) {
                return true;
            }
        }
    }

    return false;
}
