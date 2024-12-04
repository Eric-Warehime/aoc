use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    part1()?;
    part2()?;
    Ok(())
}

fn part2() -> Result<(), std::io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/input.txt");
    let buff_reader = BufReader::new(fs::File::open(path)?);
    let mut similarity = BTreeMap::<u64, (u64, u64)>::new();
    buff_reader.lines().for_each(|line| {
        let line = line.unwrap();
        let (input1, input2) = sscanf::scanf!(line, "{}   {}", u64, u64).unwrap();
        match similarity.get(&input1) {
            None => {similarity.insert(input1, (1, 0));}
            Some(existing) => {similarity.insert(input1, (existing.0+1,existing.1));}
        }
        match similarity.get(&input2) {
            None => {similarity.insert(input2, (0,1));}
            Some(existing) => {similarity.insert(input2, (existing.0,existing.1+1));}
        }
    });
    let mut similarity_score = 0;
    for key in similarity.keys() {
        let (left, right) = similarity.get(&key).unwrap();
        similarity_score += left * right * key;
    }
    println!("List similarity: {}", similarity_score);
    Ok(())
}

fn part1() -> Result<(), std::io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/input.txt");
    let buff_reader = BufReader::new(fs::File::open(path)?);
    let mut list1 = Vec::<u64>::new();
    let mut list2 = Vec::<u64>::new();
    buff_reader.lines().for_each(|line| {
        let line = line.unwrap();
        let (input1, input2) = sscanf::scanf!(line, "{}   {}", u64, u64).unwrap();
        list1.push(input1);
        list2.push(input2);
    });
    list1.sort_unstable();
    list2.sort_unstable();
    let mut distance = 0;
    for i in 0..list1.len() {
        distance += list1[i].abs_diff(list2[i])
    }
    println!("List distance: {}", distance);
    Ok(())
}