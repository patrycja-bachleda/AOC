extern crate multimap;

use std::fs::read_to_string;
use multimap::MultiMap;

fn d5_1(line: Vec<i32>, rules: MultiMap<i32, i32>) -> bool {
    for (i, num) in line.iter().enumerate() {
        let values = rules.get_vec(num);

        if !values.is_none() {
            for (j, second_num) in line.iter().enumerate() {
                if values.unwrap().iter().any(|val| val == second_num) {
                    if i > j {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn d5_2(line: Vec<i32>, rules: MultiMap<i32, i32>) -> Vec<i32>{
    let mut temp = line[0];
    let mut new_line = line.clone();

    for (i, num) in new_line.clone().iter().enumerate() {
        let values = rules.get_vec(num);

        if !values.is_none() {
            for (j, second_num) in new_line.clone().iter().enumerate() {
                if values.unwrap().iter().any(|val| val == second_num) {
                    if i > j {
                        temp = new_line[i];
                        new_line[i] =  new_line[j];
                        new_line[j] = temp;
                    }
                }
            }
        }
    }
    new_line
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let input: Vec<&str> = input.split("\n\n").collect();
    let mut rules = MultiMap::new();

    for rule in input[0].split("\n") {
        let rule: Vec<&str> = rule.split("|").collect();
        let key: i32 = rule[0].parse::<i32>().unwrap();
        let value: i32 = rule[1].parse::<i32>().unwrap();
        rules.insert(key, value);
    }

    let input = input[1].split("\n").map(|line| {
        let line: Vec<&str> = line.split(",").collect();
        let line: Vec<i32> = line.iter().map(|l| l.parse().unwrap()).collect();
        line
    }).collect::<Vec<Vec<i32>>>();

    let mut sum: i32 = 0;
    let mut sum1: i32 = 0;

    for line in input {
        if d5_1(line.clone(), rules.clone()) {
            sum += line[line.len() / 2];
        } else {
            let new_line = d5_2(line.clone(), rules.clone());
            sum1 += new_line[new_line.len() / 2];
        }
    }

    println!("{}", sum);
    println!("{}", sum1);
}
