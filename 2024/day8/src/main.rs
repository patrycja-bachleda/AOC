use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn d8_1(table: Vec<Vec<char>>, antennas: HashMap<char, HashSet<(i32, i32)>>) {
    // Find all combinations and add antinodes
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let max_x = table[0].len() as i32;
    let max_y = table.len() as i32;

    for (_k, positions) in antennas {
        for pair in positions.iter().combinations(2) {
            let p1 = *pair[0];
            let p2 = *pair[1];

            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;

            let higher_node = (p1.0 - dx, p1.1 - dy);
            let lower_node = (p2.0 + dx, p2.1 + dy);

            if higher_node.0 >= 0 && higher_node.0 < max_y && higher_node.1 >= 0 && higher_node.1 < max_x {
                antinodes.insert(higher_node);
            }

            if lower_node.0 >= 0 && lower_node.0 < max_y && lower_node.1 >= 0 && lower_node.1 < max_x {
                antinodes.insert(lower_node);
            }
        }
    }

    println!("{:?}", antinodes.len());
}

fn d8_2(table: Vec<Vec<char>>, antennas: HashMap<char, HashSet<(i32, i32)>>) {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let max_x = table[0].len() as i32;
    let max_y = table.len() as i32;

    for (_k, positions) in antennas {
        for pair in positions.iter().combinations(2) {
            let p1 = *pair[0];
            let p2 = *pair[1];

            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;

            for y3 in 0..max_y {
                for x3 in 0..max_x {
                    if (p1.0 * (p2.1 - y3) + p2.0 * (y3 - p1.1) + x3 * (p1.1 - p2.1)).abs() == 0 {
                        antinodes.insert((x3, y3));
                    }
                }
            }
        }
    }

    println!("{:?}", antinodes.len());
}

fn main() {
    let test = std::fs::read_to_string("./input.txt").unwrap();

    let mut table: Vec<Vec<char>> = Vec::new();

    test.lines()
        .map(|line| line.chars().filter(|&c| c != ' ').collect::<Vec<char>>())
        .for_each(|line| {
            table.push(line);
        });

    // Find antennas
    let mut antennas: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    for row in 0..table.len() {
        for col in 0..table[0].len() {
            if table[row][col] != '.' {
                antennas
                    .entry(table[row][col])
                    .or_insert_with(HashSet::new)
                    .insert((row as i32, col as i32));
            }
        }
    }

    println!("{:?}", antennas);

    d8_1(table.clone(), antennas.clone());
    d8_2(table.clone(), antennas.clone());
}