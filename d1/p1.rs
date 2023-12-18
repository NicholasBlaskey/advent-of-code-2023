use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    //let path = Path::new("t2.in");
    let path = Path::new("p1.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };

    let mut cube_amounts: HashMap<String, i32> = HashMap::new();
    cube_amounts.insert("red".to_string(), 12);
    cube_amounts.insert("green".to_string(), 13);
    cube_amounts.insert("blue".to_string(), 14);

    let mut possible_sum = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let game_id: i32 = line.split(":").next().unwrap().split(" ").last().unwrap().parse().unwrap();

                let mut possible = true;

                let sub_games = line.split(": ").last().unwrap().split("; ");
                for sub_game in sub_games {
                    let mut cur_game_cubes = cube_amounts.clone();
                    let sub_cubes = sub_game.split(", ");
                    for sub_cube in sub_cubes {
                        let cube_amount: i32 = sub_cube.split(" ").next().unwrap().parse().unwrap();
                        let cube_color = sub_cube.split(" ").last().unwrap();
                        let cubes_left = cur_game_cubes.get(cube_color).unwrap_or(&0) - cube_amount;

                        cur_game_cubes.insert(cube_color.to_string(), cubes_left);


                        for (_, cube_amount) in cur_game_cubes.clone().into_iter() {
                            if cube_amount < 0 {
                                possible = false;
                            }
                        }
                    }
                }

                if possible {
                    possible_sum += game_id;
                }
            }
            Err(e) => println!("ERROR: {}", e),
        }
    }

    println!("{}", possible_sum)
}
