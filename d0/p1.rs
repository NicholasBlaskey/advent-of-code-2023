use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("p1.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut sum = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
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
                        None => {}
                    }
                }
                sum += firstNumber * 10 + lastNumber;
            }
            Err(e) => println!("ERROR: {}", e),
        }
    }

    println!("{}", sum)
}
