use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn d6_2(mut table: Vec<Vec<char>>) {
    let mut count = 0;
    for row in 0..table.len() {
        for col in 0..table[row].len() {
            if table[row][col] == '.' {
                let mut new_table = table.clone();
                new_table[row][col] = '0';
                if causes_loop(&new_table) {
                    count += 1;
                }
            }
        }
    }
    println!("{:?}", count);
}

fn causes_loop(table: &Vec<Vec<char>>) -> bool {
    let directions: Vec<(isize, isize)> = vec![
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];

    let mut curr_direction_index = 0;
    let mut xpos = 0;
    let mut ypos = 0;

    for row in 0..table.len() {
        for col in 0..table[row].len() {
            match table[row][col] {
                '^' => { curr_direction_index = 0; xpos = row; ypos = col; }
                '>' => { curr_direction_index = 1; xpos = row; ypos = col; }
                'v' => { curr_direction_index = 2; xpos = row; ypos = col; }
                '<' => { curr_direction_index = 3; xpos = row; ypos = col; }
                _ => {}
            }
        }
    }

    let mut visited = HashSet::new();

    loop {
        let prev_x = xpos;
        let prev_y = ypos;

        while xpos < table.len() && ypos < table[0].len() && table[xpos][ypos] != '#' && table[xpos][ypos] != '0' {
            if visited.contains(&(xpos, ypos, curr_direction_index)) {
                return true;
            }
            visited.insert((xpos, ypos, curr_direction_index));
            xpos = (xpos as isize + directions[curr_direction_index].0) as usize;
            ypos = (ypos as isize + directions[curr_direction_index].1) as usize;

            if xpos >= table.len() || ypos >= table[0].len() {
                break;
            }
        }

        if xpos >= table.len() || ypos >= table[0].len() || (prev_x == xpos && prev_y == ypos) {
            break;
        }

        xpos = (xpos as isize - directions[curr_direction_index].0) as usize;
        ypos = (ypos as isize - directions[curr_direction_index].1) as usize;
        curr_direction_index = (curr_direction_index + 1) % 4; // turn the direction to the right
    }

    false
}

fn d6_1(mut table: Vec<Vec<char>>) {
    let directions: Vec<(isize, isize)> = vec![
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];

    let mut curr_direction_index = 0;
    let mut xpos = 0;
    let mut ypos = 0;
    let mut sum = 0;
    for row in 0..table.len() {
        for col in 0..table[row].len() {
            match table[row][col] {
                '^' => { curr_direction_index = 0; xpos = row; ypos = col; }
                '>' => { curr_direction_index = 1; xpos = row; ypos = col; }
                'v' => { curr_direction_index = 2; xpos = row; ypos = col; }
                '<' => { curr_direction_index = 3; xpos = row; ypos = col; }
                _ => {}
            }
        }
    }

    loop {
        while xpos < table.len() && ypos < table[0].len() && table[xpos][ypos] != '#' {
            table[xpos][ypos] = 'X';
            xpos = (xpos as isize + directions[curr_direction_index].0) as usize;
            ypos = (ypos as isize + directions[curr_direction_index].1) as usize;

            if xpos >= table.len() || ypos >= table[0].len() {
                break;
            }

            if table[xpos][ypos] != 'X' && table[xpos][ypos] != '#' {
                sum += 1;
            }
        }

        if xpos >= table.len() || ypos >= table[0].len() {
            break;
        }

        xpos = (xpos as isize - directions[curr_direction_index].0) as usize;
        ypos = (ypos as isize - directions[curr_direction_index].1) as usize;
        curr_direction_index = (curr_direction_index + 1) % 4; // turn the direction to the right
    }

    println!("{:?}", sum+1);
}

fn main() {
    let test = read_to_string("./input.txt").unwrap();
    // let test = "....#.....
    //             .........#
    //             ..........
    //             ..#.......
    //             .......#..
    //             ..........
    //             .#..^.....
    //             ........#.
    //             #.........
    //             ......#...";
    let mut table: Vec<Vec<char>> = Vec::new();

    test.lines().map(|line| {
        line.chars().filter(|&c| c != ' ').collect::<Vec<char>>()
    }).for_each(|line| {
        table.push(line);
    });

    d6_1(table.clone());
    d6_2(table.clone());
}