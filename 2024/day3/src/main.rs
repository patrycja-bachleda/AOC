use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use regex::Regex;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn d3_1(lines: Vec<String>){
    let re = Regex::new(r"(^|do\(\)).+?(don't\(\)|$)").unwrap();

    let mut sum = 0;

    for line in lines{
        for (_, [num1, num2]) in re.captures_iter(&line).map(|c| c.extract()) {
            sum += num1.parse::<u64>().unwrap() * num2.parse::<u64>().unwrap();
        }
    }

    println!("{}", sum);
}

fn d3_2(lines: Vec<String>){
    let input = lines.join("");
    let re2 = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do(n't)?\(\)").unwrap();
    let mut enabled = true;
    let sum: isize = re2.captures_iter(&input).map(|c| {
        if &c[0] == "do()" {
            enabled = true;
            return 0;
        } else if &c[0] == "don't()" {
            enabled = false;
            return 0;
        } else if !enabled {
            return 0;
        } else {
            let a: isize = c[1].parse().unwrap();
            let b: isize = c[2].parse().unwrap();
            return a * b;
        }
    }).sum();

    println!("{}", sum);
}

fn main() {
    let lines = lines_from_file("./input.txt");
    // d3_1(lines);
    d3_2(lines);
}