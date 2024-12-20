use std::fs::read_to_string;
use regex::Regex;

fn d17_1(mut a: i32, mut b: i32, mut c:i32, program: Vec<i32>) -> Vec<i32> {
    // println!("{} {}  {} {:?}", a, b, c, program);
    let mut result: Vec<i32> = Vec::new();
    let mut i = 0;

    // for (mut i, mut instruction) in program.iter().step_by(2).enumerate()

    while i < program.len()-1{
        match program[i]{
            0 => {
                a = a / 2_i32.pow(find_combo(a, b, c, program[i+1]) as u32);
            }, //truncate to int and then write to A
            1 => {
                b = b ^ program[i+1];
            }, // then write to B
            2 => {
                b = find_combo(a, b, c, program[i+1]) % 8;
            }, // then write to B
            3 => {
                if a != 0 { i = program[i + 1] as usize; continue; }
            }, // do not increase (ig repeat this)
            4 => {
                b = b ^ c;
            },
            5 => {
                result.push(find_combo(a, b, c, program[i+1]) % 8);
                // println!("{:?}", result);
            }, // then outputs values
            6 => {
                b = a / 2_i32.pow(find_combo(a, b, c, program[i+1]) as u32);
            }, // truncate to int and then write to B
            7 => {
                c = a / (2_i32.pow(find_combo(a, b, c, program[i+1]) as u32));
            }, // truncate to int and then write to C
            _ => {}
        }
        i += 2;
    }

    // println!("{} {} {} {:?}", a, b, c, result);

    result
}

fn find_combo(a: i32, b: i32, c: i32, operand: i32) -> i32{
    let result = match operand{
        0 | 1| 2 | 3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => 0
    };
    result
}

fn main() {
    // let test =    "Register A: 117440
    //                     Register B: 0
    //                     Register C: 0
    //
    //                     Program: 0,3,5,4,3,0";

    let test = read_to_string("./input.txt").unwrap();

    let re = Regex::new(r"Register A: ([0-9]+)\s+Register B: ([0-9]+)\s+Register C: ([0-9]+)\s+Program: (.*)").unwrap();

    for (_, [a, b, c, program]) in re.captures_iter(&test).map(|c| c.extract()){
        for i in 100000000..u32::MAX{
            let old: Vec<i32> = program.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
            let new = d17_1(i as i32, b.parse::<i32>().unwrap(), c.parse::<i32>().unwrap(), old.clone());
            if new == old.clone(){
                println!("{}", i);
                break;
            }
        }
    }
}

// 0 = adv (division) num = A register, den = 2^operand
//
