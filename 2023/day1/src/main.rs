use std::fs::read_to_string;
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

fn d1_1(lines: Vec<Vec<char>>){ // abs3def
    //1abc3
    let mut sum = 0;

    for line in lines{
        let mut left = 'a';
        let mut index = 0;
        let mut right = 'a';

        // abc1
        // 1abc

        for i in 0..line.len(){
            if line[i].is_numeric() && left != 'a' && i < index{
                left = line[i];
            }

            if line[line.len() - i - 1].is_numeric() && right != 'a'{
                right = line[line.len() - i - 1];
                index = line.len() - i - 1;
            }

            if right != 'a' && left != 'a'{
                break;
            }
        }
        // abd
        let mut combined = String::from("");
        if left != 'a' {combined.push(left)};
        if right != 'a' {combined.push(right)};
        sum += combined.parse::<i32>().unwrap();
    }
    println!("{}", sum);
}

fn d1_2(){

}

fn main() {
    let test = read_to_string("./input.txt").unwrap();
    let mut table: Vec<Vec<char>> = Vec::new();

    test.lines().map(|line| {
        line.chars().filter(|&c| c != ' ').collect::<Vec<char>>()
    }).for_each(|line| {
        table.push(line);
    });

    d1_1(table);
    // d1_2();
}
