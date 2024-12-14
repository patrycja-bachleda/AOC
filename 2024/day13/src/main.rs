use std::fs::read_to_string;
use regex::Regex;
fn d13_1(a: (i128, i128), b: (i128, i128), prize: (i128, i128)) -> i128{
    let det = (a.0 * b.1 - b.0 * a.1) as f64;

    if det == 0.0 {
        return 0;
    }

    let press_a  = (b.1 * prize.0 - b.0 * prize.1) as f64 / det;
    let press_b  = (-a.1 * prize.0 + a.0 * prize.1) as f64 / det;

    if press_a % 1.0 != 0.0 || press_b % 1.0 != 0.0{
        return 0;
    }
    press_a as i128 * 3 + press_b as i128
}

fn main() {
    let test = read_to_string("input.txt").unwrap();

    let re = Regex::new(r"Button A: X([+-][0-9]+), Y([+-][0-9]+)\s+Button B: X([+-][0-9]+), Y([+-][0-9]+)\s+Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let mut sum = 0;

    for (_, [num_a1, num_a2, num_b1, num_b2, num_p1, num_p2]) in re.captures_iter(&test).map(|c| c.extract()){
        sum += d13_1(
            (num_a1.parse().unwrap(), num_a2.parse().unwrap()),
            (num_b1.parse().unwrap(), num_b2.parse().unwrap()),
            (num_p1.parse::<i128>().unwrap(), num_p2.parse::<i128>().unwrap()));
    }

    println!("{}", sum);
    sum = 0;

    for (_, [num_a1, num_a2, num_b1, num_b2, num_p1, num_p2]) in re.captures_iter(&test).map(|c| c.extract()){
        sum += d13_1(
            (num_a1.parse().unwrap(), num_a2.parse().unwrap()),
            (num_b1.parse().unwrap(), num_b2.parse().unwrap()),
            (num_p1.parse::<i128>().unwrap() + 10000000000000, num_p2.parse::<i128>().unwrap() + 10000000000000));
    }

    println!("{}", sum);


}
