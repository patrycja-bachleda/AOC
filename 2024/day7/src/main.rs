use std::fs::read_to_string;

fn d7_2(line: &str) -> i64 {
    let parts: Vec<&str> = line.split(":").collect();
    let goal: i64 = parts[0].trim().parse().unwrap();
    let values: Vec<i64> = parts[1]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let operators = ["*", "+", "||"];
    let mut results = vec![values[0]];

    for &value in &values[1..] {
        let mut new_results = vec![];
        for &result in &results {
            for &operator in &operators {
                new_results.push(
                    match operator {
                        "*" => result * value,
                        "+" => result + value,
                        "||" => (result.to_string() + &value.to_string()).parse().unwrap(),
                        _ => 0,
                    })
            }
        }
        results = new_results;
    }

    if results.contains(&goal){
        return goal;
    }
    0
}

fn d7_1(line: &str) -> i64 {
    let parts: Vec<&str> = line.split(":").collect();
    let goal: i64 = parts[0].trim().parse().unwrap();
    let values: Vec<i64> = parts[1]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let operators = ['*', '+'];
    let mut results = vec![values[0]];

    for &value in &values[1..] {
        let mut new_results = vec![];
        for &result in &results {
            for &operator in &operators {
                new_results.push(
                    match operator {
                        '*' => result * value,
                        '+' => result + value,
                        _ => 0,
                    })
            }
        }
        results = new_results;
    }

    if results.contains(&goal){
        return goal;
    }
    0
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let mut sum = 0;
    let mut sum1 = 0;

    for line in input.split("\n") {
        sum += d7_1(line);
        sum1 += d7_2(line);
    }

    println!("{:?}", sum);
    println!("{:?}", sum1);
}