use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn is_safe(line: &str) -> bool {
    let nums: Vec<u128> = line
        .split_whitespace()
        .map(|num| num.parse::<u128>().unwrap())
        .collect();

    let state = if nums[0] > nums[1] { "down" } else if nums[1] > nums[0]{ "up" } else{ return false };

    for i in 0..nums.len() - 1 {
        let diff = if nums[i] > nums[i + 1] { nums[i] - nums[i + 1] } else { nums[i + 1] - nums[i] };
        let state1 = if nums[i] > nums[i+1] { "down" } else if nums[i+1] > nums[i]{ "up" } else{ return false };

        if state != state1 || diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn is_safe_removed(line: &str) -> bool{
    let nums: Vec<u128> = line
    .split_whitespace()
    .map(|num| num.parse::<u128>().unwrap())
    .collect();

    for i in 0..nums.len() {
        let mut copy = nums.clone();
        copy.remove(i);
        let result = copy.iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        if is_safe(&result) {
            return true;
        }
    }

    return false;
}

fn main() {
    let lines = lines_from_file("./input.txt");
    let mut sum = 0;

    for line in lines.clone() {
        if is_safe(&line) {
            sum += 1;
        }
    }

    println!("{}", sum);

    let mut sum = 0;

    for line in lines.clone() {
        if is_safe_removed(&line) {
            sum += 1;
        }
    }

    println!("{}", sum);
}