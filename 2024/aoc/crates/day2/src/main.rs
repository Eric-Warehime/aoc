use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    part1()?;
    Ok(())
}

fn level_is_unsafe(lvl: Vec<i64>) -> bool {
    let mut sign = 0_i64;
    let mut prev = 0_i64;
    for i in 0..lvl.len() {
        if i == 0 {
            prev = lvl[i];
            continue
        }
        if i == 1 {
            if prev < lvl[i] {
                sign = -1
            } else if prev > lvl[i] {
                sign = 1
            } else {
                return true
            }
        }
        let change = (prev - lvl[i]) * sign;
        if change < 1 || change > 3 {
            return true
        }
        prev = lvl[i]
    }
    false
}


fn part1() -> Result<(), std::io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/input.txt");
    let buff_reader = BufReader::new(fs::File::open(path)?);
    let mut safe_levels = 0;
    buff_reader.lines().for_each(|line| {
        let line = line.unwrap();
        let level = line.split_whitespace()
            .filter_map(|v| v.parse::<i64>().ok())
            .collect::<Vec<i64>>();
        if !level_is_unsafe(level) {
            safe_levels += 1;
        }
    });
    println!("Safe levels: {}", safe_levels);
    Ok(())
}