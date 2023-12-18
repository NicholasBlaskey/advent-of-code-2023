use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::cmp;

fn main() {
    //let path = Path::new("t2.in");
    let path = Path::new("p1.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };

    let mut possible_sum = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut cube_amounts: HashMap<String, i32> = HashMap::new();

                let sub_games = line.split(": ").last().unwrap().split("; ");
                for sub_game in sub_games {
                    let sub_cubes = sub_game.split(", ");
                    for sub_cube in sub_cubes {
                        let cube_amount: i32 = sub_cube.split(" ").next().unwrap().parse().unwrap();
                        let cube_color = sub_cube.split(" ").last().unwrap();



                        let cubes_max = cmp::max(
                            cube_amounts.get(cube_color).unwrap_or(&0),
                            &cube_amount,
                        );
                        cube_amounts.insert(cube_color.to_string(), *cubes_max);

                    }
                }


                let mut power = 1;
                for (_, cube_amount) in cube_amounts.clone().into_iter() {
                    power *= cube_amount;
                }
                possible_sum += power;
            }
            Err(e) => println!("ERROR: {}", e),
        }
    }

    println!("{}", possible_sum)
}
