use std::collections::HashMap;

fn d19_1(comb: &str, towels: &[&str], memo: &mut HashMap<String, bool>) -> bool {
    if comb.is_empty() {
        return true;
    }

    for &towel in towels {
        if comb.starts_with(towel) {
            let remaining = &comb[towel.len()..];
            if d19_1(remaining, towels, memo) {
                memo.insert(comb.to_string(), true);
                return true;
            }
        }
    }

    memo.insert(comb.to_string(), false);
    false
}

fn main() {
    let test = std::fs::read_to_string("./input.txt").unwrap();

    // Parse the input.
    let parts: Vec<&str> = test.split("\n\n").collect();
    let towels: Vec<&str> = parts[0].split(", ").collect();
    let combinations: Vec<&str> = parts[1].lines().collect();

    // Count possible combinations.
    let mut memo = HashMap::new();
    let result =  combinations
        .iter()
        .filter(|&&design| d19_1(design, &towels, &mut memo))
        .count();
    println!("Number of possible combinations: {}", result);
}
