use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;

fn main() {
    let path = Path::new("p1.in");
    //let path = Path::new("t2.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|e| e.ok()).collect();

    let seed_to_soil = parse_map(&lines, "seed-to-soil");
    let soil_to_fertilizer = parse_map(&lines, "soil-to-fertilizer");
    let fertilizer_to_water = parse_map(&lines, "fertilizer-to-water");
    let water_to_light = parse_map(&lines, "water-to-light");
    let light_to_temperature = parse_map(&lines, "light-to-temperature");
    let temperature_to_humidity = parse_map(&lines, "temperature-to-humidity");
    let humidity_to_location = parse_map(&lines, "humidity-to-location");

    let mut min_location = i64::MAX;
    for seed in lines[0].split("seeds: ").last().unwrap().split(" ") {
        let soil = lookup_map(&seed_to_soil, seed.parse().unwrap());
        let fertilizer = lookup_map(&soil_to_fertilizer, soil);
        let water = lookup_map(&fertilizer_to_water, fertilizer);
        let light = lookup_map(&water_to_light, water);
        let temperature = lookup_map(&light_to_temperature, light);
        let humidity = lookup_map(&temperature_to_humidity, temperature);
        let location = lookup_map(&humidity_to_location, humidity);


        min_location = cmp::min(min_location, location);
    }

    println!("{}", min_location);
}

fn parse_map(lines: &Vec<String>, map_name: &str) -> Vec<(i64, i64, i64)> {
    let mut found_map = false;
    let mut map: Vec<(i64, i64, i64)> = Vec::new();
    for l in lines {
        if l.contains(map_name) {
            found_map = true;
            continue;
        }

        if found_map {
            if l == "" {
                break
            }

            // TODO we should define a type...
            let mut iter = l.split(" ");
            let dest_range_start = iter.next().unwrap().parse().unwrap();
            let source_range_start = iter.next().unwrap().parse().unwrap();
            let range_length = iter.next().unwrap().parse().unwrap();
            map.push((dest_range_start, source_range_start, range_length));
        }
    }

    return map;
}

fn lookup_map(map: &Vec<(i64, i64, i64)>, value: i64) -> i64 {
    for tuple in map {
        if value >= tuple.1 && value <= (tuple.1 + tuple.2) {
            return tuple.0 - tuple.1 + value;
        }
    }

    return value
}
