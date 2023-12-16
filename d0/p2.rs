use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let path = Path::new("p1.in");
    //let path = Path::new("t2.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut wordToDigitVal: HashMap<String, u32> = HashMap::new();
    wordToDigitVal.insert("one".to_string(), 1);
    wordToDigitVal.insert("two".to_string(), 2);
    wordToDigitVal.insert("three".to_string(), 3);
    wordToDigitVal.insert("four".to_string(), 4);
    wordToDigitVal.insert("five".to_string(), 5);
    wordToDigitVal.insert("six".to_string(), 6);
    wordToDigitVal.insert("seven".to_string(), 7);
    wordToDigitVal.insert("eight".to_string(), 8);
    wordToDigitVal.insert("nine".to_string(), 9);

    let mut sum = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut i = 0; // TODO swap to iterator enumarate
                let mut firstNumber = 0;
                let mut setFirst = false;
                let mut lastNumber = 0;
                for c in line.chars() {
                    let v = c.to_digit(10);
                    match v {
                        Some(v) => {
                            if !setFirst {
                                firstNumber = v;
                                setFirst = true;
                            }
                            lastNumber = v;
                        },
                        None => {
                            // Character case.
                            for (word, digitVal) in wordToDigitVal.clone().into_iter() {
                                if i < word.chars().count() - 1 {
                                    continue;
                                }

                                let pastChars = &line[((i+1)-word.chars().count())..i+1];
                                if pastChars == word {
                                    //println!("{} {} {}", line, i, pastChars);

                                    if !setFirst {
                                        firstNumber = digitVal;
                                        setFirst = true;
                                    }
                                    lastNumber = digitVal;
                                }

                            }
                        }
                    }
                    i += 1;
                }

                sum += firstNumber * 10 + lastNumber;
            }
            Err(e) => println!("ERROR: {}", e),
        }
    }

    println!("{}", sum)
}
