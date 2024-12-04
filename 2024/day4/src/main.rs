use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::fs::read_to_string;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn check_direction(table: &Vec<Vec<char>>, xpos: usize, ypos: usize, dx: isize, dy: isize) -> bool {
    let xmas: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let mut x = xpos as isize;
    let mut y = ypos as isize;

    for &ch in &xmas {
        if x < 0 || y < 0 || x >= table.len() as isize || y >= table[0].len() as isize {
            return false;
        }
        if table[x as usize][y as usize] != ch {
            return false;
        }
        x += dx;
        y += dy;
    }
    true
}

fn check_xmas_pattern(table: &Vec<Vec<char>>, xpos: usize, ypos: usize) -> bool {
    // let directions: Vec<(isize, isize)> = vec![
    //     (1, 1),  // down-right
    //     (1, -1), // down-left
    //     (-1, 1), // up-right
    //     (-1, -1),// up-left
    // ];
    //
    // for &(dx, dy) in &directions {
    //     if xpos as isize + 2 * dx >= 0 && xpos as isize + 2 * dx < table.len() as isize &&
    //         ypos as isize + 2 * dy >= 0 && ypos as isize + 2 * dy < table[0].len() as isize &&
    //         xpos as isize - 2 * dx >= 0 && xpos as isize - 2 * dx < table.len() as isize &&
    //         ypos as isize - 2 * dy >= 0 && ypos as isize - 2 * dy < table[0].len() as isize {
    //         if table[xpos][ypos] == 'A' &&
    //             table[(xpos as isize + dx) as usize][(ypos as isize + dy) as usize] == 'M' &&
    //             table[(xpos as isize + 2 * dx) as usize][(ypos as isize + 2 * dy) as usize] == 'S' &&
    //             table[(xpos as isize - dx) as usize][(ypos as isize - dy) as usize] == 'M' &&
    //             table[(xpos as isize - 2 * dx) as usize][(ypos as isize - 2 * dy) as usize] == 'S' {
    //             return true;
    //         }
    //     }
    // }
    if xpos <= 0 || xpos >= table.len() - 1 || ypos <= 0 || ypos >= table[0].len() - 1 {
        return false;
    }

    // m  m
    //   a
    // s  s

    // s  s
    //   a
    // m  m

    // m  s
    //   a
    // m  s

    // s  m
    //   a
    // s  m


    if table[xpos][ypos] == 'A'{
        let tl = table[xpos - 1][ypos + 1];
        let tr = table[xpos + 1][ypos + 1];
        let bl = table[xpos - 1][ypos - 1];
        let br = table[xpos + 1][ypos - 1];

        if tl == 'M' && br == 'S' && tr == 'M' && bl == 'S'  ||
            tl == 'S' && br == 'M' && tr == 'S' && bl == 'M'  ||
            tl == 'M' && br == 'S' && tr == 'S' && bl == 'M'  ||
            tl == 'S' && br == 'M' && tr == 'M' && bl == 'S'
            {
            return true;
        }
    }

    false
}

fn d4_2(table: &Vec<Vec<char>>) {
    let mut count = 0;

    for xpos in 0..table.len() {
        for ypos in 0..table[xpos].len() {
            if check_xmas_pattern(table, xpos, ypos) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn d4_1(table: &Vec<Vec<char>>) {
    let directions: Vec<(isize, isize)> = vec![
        (0, 1),  // right
        (1, 0),  // down
        (1, 1),  // down-right
        (1, -1), // down-left
        (0, -1), // left
        (-1, 0), // up
        (-1, -1),// up-left
        (-1, 1), // up-right
    ];

    let mut count = 0;

    for xpos in 0..table.len() {
        for ypos in 0..table[xpos].len() {
            for &(dx, dy) in &directions {
                if check_direction(table, xpos, ypos, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn main() {
    let test = read_to_string("./input.txt").unwrap();
    let mut table: Vec<Vec<char>> = Vec::new();

    test.lines().map(|line| {
        line.chars().filter(|&c| c != ' ').collect::<Vec<char>>()
    }).for_each(|line| {
        table.push(line);
    });

    d4_1(&table);
    d4_2(&table);
}