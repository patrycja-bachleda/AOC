use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::io::stdin;
use std::collections::HashMap;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn d1_1(lines: Vec<String>){
    let mut left_side: Vec<u128> = vec![];
    let mut right_side: Vec<u128> = vec![];

    for line in lines {
        let mut l = line.split("   ");
        let a = l.next().unwrap().parse::<u128>().unwrap();
        left_side.push(a);
        let b = l.next().unwrap().parse::<u128>().unwrap();
        right_side.push(b);
    }
    let mut sum = 0;
    left_side.sort();
    right_side.sort();
    for i in 0..left_side.len() {
        let a = left_side[i];
        let b = right_side[i];
        sum += if a > b { a - b } else { b - a }
    }
    println!("{}", sum);
}

fn d1_2(){
    let mut left_side = Vec::new();
    let mut right_side = HashMap::<u128, u128>::new();
    stdin()
        .lines()
        .filter_map(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut l = l.split("   ");
            let left = l.next().unwrap().parse::<u128>().unwrap();
            let right = l.next().unwrap().parse::<u128>().unwrap();
            (left, right)
        })
        .for_each(|(left, right)| {
            left_side.push(left);
            *right_side.entry(right).or_default() += 1;
        });
    let sum = left_side
        .iter()
        .fold(0, |acc, x| acc + x * right_side.get(&x).unwrap_or(&0));
    println!("{}", sum);
}

fn main() {
    let lines = lines_from_file("./input.txt");
    d1_1(lines);
    d1_2();
}
