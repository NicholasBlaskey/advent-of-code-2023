// taken from https://topaz.github.io/paste/#XQAAAQDrBAAAAAAAAAApj0BGg5FxsnVtuf36p4TsBPiq87tED9vIGZk9flQeV0ZbOEiHh3HJPDBEQ5vwW8Kg7tUbyzpTt2KGKbGmzYpRlUoeERKRkKDsgMzZsUlSFGAI/eSYvjPOTEy3J7iSg/oBFPLj/9S9tXDW4OAvuGpgz6D4jtx37lbZ4pLvjkByA+rWIPhkU2vh9qJzG8KuSIkn+uLYFJYKxI6WjRlTA33K+N5PAxnTph3YBya1koPdqbLcHiSQspmPS4k5aFL/xgyWRS5EEsauBHDVc8lhLhbBsnGd6aKEiH4rlLc6GtiOl8b4JfnI1OXNb9WRzkybCe4gpKhKKn8cqUrVNpVKwH29myuY9/xonKFF0b1pEKqNd6ylavDjX3rOo17PwAPnIH7emJxpJjgqJs5C/LCwaqFm4Gpy3OC56CIsjmFN7fFKXn6zzv0/qqFe9QWtYXuTWISSPe/cOu9kV8+k1XF7iVnLBcSiNlOvyGQWa1tPKtDDMoEkpLJYkC/x7HZfFOtocyVzAHcSO9Gkpyl8In7F1lnCZipIJpsOelnoFagGBLTOW9eLqPRMRGlxt4exV6Db1CEcgav5Uwr+G9bnOM2Ple1PPOAUXMhustAJo+snyXkjTd7o0MB9rKmC70eU+xJf2xWmlH1VFoKk4PCprbxl3D+Q/0E8eaJ0BzMdkTewRMGEg2/WgFbn4LDFObGqduyXn7TqE9irJswy0xe57KgFW/BwltvScyWw0brbhVjhzKH6xrSfwVa79IwdJkEDGjDbwPQsjH1e5PPtqm/XYEMa+xvnqK8wlIAe6IP1J1MMftpie0RRjqTR1JuihKKK0sEipMA01IGAy2dbV/OefLY1FBDVeI//fvTaAA==
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let path = Path::new("p1.in");
    //let path = Path::new("t2.in");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };


    let mut lines: Vec<String> = BufReader::new(file).lines().filter_map(|e| e.ok()).collect();
    lines.insert(0, ".".repeat(lines[0].len()));
    lines.push(".".repeat(lines[0].len()));
    for i in 0..(lines.len()) {
        let new = ".".to_owned() + &lines[i] + ".";
        lines[i] = new;
    }

    let mut found = false;
    let mut i = 0;
    let mut j = 0;
    for l in &lines {
        j = 0;
        for c in l.chars() {
            if c == 'S' {
                found = true;
                break
            }
            j += 1;
        }
        if found {
            break;
        }

        i += 1;
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    visited.insert((i, j));
    i -= 1; // this is for actual probleme TODO unhardcode this first step.
    let s_valid = true;

    loop {
        visited.insert((i, j));

        let north = (i-1, j);
        let south = (i+1, j);

        let west = (i, j-1);
        let east = (i, j+1);

        if lines[i].chars().nth(j).unwrap() == '|' {
            (i, j) = pick_between(&lines, &mut visited, north, south);
        } else if lines[i].chars().nth(j).unwrap() == '-' {
            (i, j) = pick_between(&lines, &mut visited, east, west);
        } else if lines[i].chars().nth(j).unwrap() == 'L' {
            (i, j) = pick_between(&lines, &mut visited, north, east);
        } else if lines[i].chars().nth(j).unwrap() == 'J' {
            (i, j) = pick_between(&lines, &mut visited, north, west);
        } else if lines[i].chars().nth(j).unwrap() == '7' {
            (i, j) = pick_between(&lines, &mut visited, south, west);
        } else if lines[i].chars().nth(j).unwrap() == 'F' {
            (i, j) = pick_between(&lines, &mut visited, south, east);
        } else if lines[i].chars().nth(j).unwrap() == '.' {
        } else if lines[i].chars().nth(j).unwrap() == 'S' {
            break
        }
    }

    let mut count = 0;
    i = 0;
    for l in &lines {
        let mut parity = 0;
        j = 0;
        for c in l.chars() {
            if visited.contains(&(i, j)) {
                if c == '|' || c == 'J' || c == 'L' || (c == 'S' && s_valid) {
                    parity += 1;
                }
            } else {
                if parity % 2 != 0 {
                    count += 1;
                }
            }
            j += 1;
        }
        i += 1;
    }

    println!("{}", count);
}

fn pick_between(lines: &Vec<String>, visited: &mut HashSet<(usize, usize)>, a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    if visited.contains(&a) {
        return b;
    } else if visited.contains(&b) {
        return a;
    }

    if lines[a.0].chars().nth(a.1).unwrap() == '.' || lines[a.0].chars().nth(a.1).unwrap() == 'S' {
        return b;
    }
    return a;
}

