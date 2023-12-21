// approach taken from https://github.com/weibell/AoC2023-python/blob/main/day05/part2.py
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

    let seed_to_soil = parse_map(&lines, "seed-to-soil");
    let soil_to_fertilizer = parse_map(&lines, "soil-to-fertilizer");
    let fertilizer_to_water = parse_map(&lines, "fertilizer-to-water");
    let water_to_light = parse_map(&lines, "water-to-light");
    let light_to_temperature = parse_map(&lines, "light-to-temperature");
    let temperature_to_humidity = parse_map(&lines, "temperature-to-humidity");
    let humidity_to_location = parse_map(&lines, "humidity-to-location");

    let seed_vec: Vec<&str> = lines[0].split("seeds: ").last().unwrap().split(" ").collect();

    let mut seed_ranges: Vec<(i64, i64)> = Vec::new();
    for i in (0..(seed_vec.len())).step_by(2) {
        let range_start = seed_vec[i].parse().unwrap();
        let range_length: i64 = seed_vec[i + 1].parse().unwrap();
        seed_ranges.push((range_start, range_start + range_length));
    }

    for v in 0..10000000 {
        let mut l = v;
        l = just_invert(&humidity_to_location, l);
        l = just_invert(&temperature_to_humidity, l);
        l = just_invert(&light_to_temperature, l);
        l = just_invert(&water_to_light, l);
        l = just_invert(&fertilizer_to_water, l);
        l = just_invert(&soil_to_fertilizer, l);
        l = just_invert(&seed_to_soil, l);

        let mut found = false;
        for r in &seed_ranges {
            if l >= r.0 && l < r.1 {
                found = true;
                break;
            }
        }

        if found {
            println!("{}", v);
            break;
        }
    }
}

fn just_invert(map: &Vec<(i64, i64, i64)>, point: i64) -> i64 {
    for m in map {
        let (dest_range_start, source_range_start, range_length) = m;
        if point >= *dest_range_start && point < dest_range_start + range_length {
            return source_range_start - dest_range_start + point;
        }
    }

    return point;
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
