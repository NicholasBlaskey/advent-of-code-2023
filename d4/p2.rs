use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    //let path = Path::new("p1.in");
    let path = Path::new("t2.in");
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

    let seed_to_fertilizer = combine_map(seed_to_soil, soil_to_fertilizer);
    let seed_to_water = combine_map(seed_to_fertilizer, fertilizer_to_water);
    let seed_to_light = combine_map(seed_to_water, water_to_light);
    let seed_to_temperature = combine_map(seed_to_light, light_to_temperature);
    let seed_to_humidity = combine_map(seed_to_temperature, temperature_to_humidity);
    let seed_to_location = combine_map(seed_to_humidity, humidity_to_location);

    let mut min_location = i64::MAX;
    /*


    let mut min_location = i64::MAX;


    // TODO delete.
    for seed in lines[0].split("seeds: ").last().unwrap().split(" ") {
        let location = lookup_map(&seed_to_location, seed.parse().unwrap());


        min_location = cmp::min(min_location, location);
    }
     */
    
    /*
    let seed_to_soil = parse_map(&lines, "seed-to-soil");
    let soil_to_fertilizer = parse_map(&lines, "soil-to-fertilizer");
    let fertilizer_to_water = parse_map(&lines, "fertilizer-to-water");
    let water_to_light = parse_map(&lines, "water-to-light");
    let light_to_temperature = parse_map(&lines, "light-to-temperature");
    let temperature_to_humidity = parse_map(&lines, "temperature-to-humidity");
    let humidity_to_location = parse_map(&lines, "humidity-to-location");


    let mut min_location = i64::MAX;
    let seed_vec: Vec<&str> = lines[0].split("seeds: ").last().unwrap().split(" ").collect();
    for i in (0..(seed_vec.len())).step_by(2) {
        let range_start = seed_vec[i].parse().unwrap();
        let range_length: i64 = seed_vec[i + 1].parse().unwrap();
        for seed in range_start..=range_start+range_length {
            let soil = lookup_map(&seed_to_soil, seed);
            let fertilizer = lookup_map(&soil_to_fertilizer, soil);
            let water = lookup_map(&fertilizer_to_water, fertilizer);
            let light = lookup_map(&water_to_light, water);
            let temperature = lookup_map(&light_to_temperature, light);
            let humidity = lookup_map(&temperature_to_humidity, temperature);
            let location = lookup_map(&humidity_to_location, humidity);


            min_location = cmp::min(min_location, location);
        }
    }
     */

    println!("{}", min_location);
}

/*
// Two points
// (from, to)
fn combine_map(a_to_b: &Vec<(i64, i64)>, b_to_c: &Vec<(i64, i64)>) -> Vec<(i64, i64, i64)> {
    

    return a_to_b;

    let mut a_to_c: Vec<(i64, i64, i64)> = Vec::new();

    // Let's get all the fucking points.
    // All the a range points. Get the start and ends and fill int he middle. I think this
    // is totally doable. We just somehow do it.
    
    // I'm lost here.
    //
    // Let's just say the problem is super nice and didn't generate overlapping intervals.
    for interval in a_to_b {
        let b_start = interval.0;
        let a_start = interval.1;
        let range_length = interval.2;

        for sub_interval in b_to_C {
            let sub_c_start = sub_interval.0;
            let sub_b_start = sub_interval.1;
            let sub_range_length = sub_interval.2;

            // Completly overlapping interval.
            if b_start >= sub_b_start && b_start + range_length <= sub_b_start + sub_range_length {

            }
        }

        // TODO the split case here is relevant.
        /*
        for sub_interval in b_to_c {
            if dest_start 

        }
        */
    }

    return a_to_c;
}
*/

fn combine_map(
    a_to_b: (Vec<i64>, HashMap<i64, i64>),
    b_to_c: (Vec<i64>, HashMap<i64, i64>)) -> (Vec<i64>, HashMap<i64, i64>) {

    let mut intervals = vec![i64::MIN, i64::MAX];
    let mut dest_starts: HashMap<i64, i64> = HashMap::new();


    println!("\n START");
    for i in 0..(a_to_b.0).len() {
        // Starts.
        let a_start = (a_to_b.0)[i];
        let b_start = (a_to_b.1).get(&a_start).unwrap();

        // Calculate ends.
        let mut a_end = a_start;
        let mut b_end = a_end;
        if i != (a_to_b.0).len() - 1 {
            a_end = (a_to_b.0)[i + 1] - 1;
            if i == 0 {
                b_end = a_end;
            } else {
                b_end = (a_end - a_start) + b_start;
            }
        }

        println!("a_start {} a_end {} b_start {} b_end {}", a_start, a_end, b_start, b_end);

        //intervals.push(a_start);

        let mut found = false;
        for sub_b_start in &b_to_c.0 {
            if sub_b_start >= b_start {
                intervals.push(a_start);
                dest_starts.insert(a_start, b_to_c.1[&sub_b_start]);


                // TODO giving up, this feels theoeritcally possible but i give up.
                // How do we update a_start?
                // Like we need a new A start that we get from this B start?
                a_start = ;
            }

            if sub_b_start >= &b_end {
                break
            }
        }
    }
    /*
    let mut critical_points_b: HashSet<i64> = HashSet::new();
    for (_, b) in a_to_b.1.clone().into_iter() {
        critical_points_b.insert(b);
    }
    for b in &b_to_c.0 {
        critical_points_b.insert(*b);
    }

    println!("COMBINED");
    for b in critical_points_b {
        println!("{}", b);
    }
    println!("DONE \n\n");
     */

    // INPUT

    // A values
    // [MIN, 0, 7, 11, 53, MAX]
    // MIN->MIN 0->42 7->57 11->0 53->49 MAX->MAX

    // B values
    // [MIN, 18, 25, MAX]
    // MIN->MIN 18->88 25->18 MAX->MAX

    // STEP 1
    
    // A values
    // [MIN, 0, 7, 11, 53, MAX]
    // MIN->MIN 0->42 7->57 11->0 53->49 MAX->MAX


    // Go from 42 to 42 + 7 in b_to_c. If we see this.

    // Essentially we just need to rewrite a to b in terms of C.
    // MIN->MIN
    
    
    

    // we want to get critical_points_a now I think.

    // I think this is wrong. Need a little more juice.

    intervals.sort();
    return (intervals, dest_starts);
}

fn parse_map(lines: &Vec<String>, map_name: &str) -> (Vec<i64>, HashMap<i64, i64>) {
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



    let mut intervals = vec![i64::MIN, i64::MAX];
    let mut dest_starts: HashMap<i64, i64> = HashMap::new();
    dest_starts.insert(i64::MIN, i64::MIN);
    dest_starts.insert(i64::MAX, i64::MAX);
    for i in 0..(map.len()) {
        let (dest_range_start, source_range_start, range_length) = map[i];

        if *intervals.last().unwrap() != source_range_start {
            intervals.push(source_range_start);
        }

        dest_starts.insert(source_range_start, dest_range_start);
    }

    intervals.sort();
    for x in intervals.clone().into_iter() {
        println!("from {} to {}", x, dest_starts.get(&x).unwrap());
    }
    println!("BREAK\n");
    
    return (intervals, dest_starts);
}

fn lookup_map(map: &Vec<(i64, i64, i64)>, value: i64) -> i64 {
    for tuple in map {
        if value >= tuple.1 && value <= (tuple.1 + tuple.2) {
            return tuple.0 - tuple.1 + value;
        }
    }

    return value
}
