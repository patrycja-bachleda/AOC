use std::collections::HashSet;
use std::fs::read_to_string;

fn create_input(test: String) -> Vec<i64>{
    let mut list: Vec<i64> = vec![];
    let mut id = 0;

    for mut i in (0..test.len()).step_by(2){
        for _j in 0..test.chars().nth(i).unwrap().to_digit(10).unwrap(){
            list.push(id);
        }

        i += 1;

        if i < test.len(){
            for _j in 0..char::to_digit(test.chars().nth(i).unwrap(), 10).unwrap(){
                list.push(-1);
            }
        }

        id += 1;
    }

    list
}

fn checksum(list: Vec<i64>){
    let mut sum = 0;

    for i in 0..list.len(){
        if list[i] != -1 {
            sum += list[i] * i as i64;
        }
    }

    println!("{}", sum);
}
fn d9_2(test: String){
    let mut list: Vec<i64> = create_input(test);
    let mut replacement = vec![];
    let mut replace = vec![];
    let mut back_pointer = list.len() - 1;
    let mut origin;
    let mut front_pointer;
    let mut modified = HashSet::new();

    while back_pointer > 0 {
        println!("{:?}",list);
        while list[back_pointer] == -1{
            back_pointer -= 1;
        }

        // find values from back
        origin = list[back_pointer];

        while list[back_pointer] == origin && back_pointer > 0{
            replacement.push(list[back_pointer]);
            back_pointer -= 1;
        }


        // find place to replace
        front_pointer = 0;
        while front_pointer < back_pointer{
            while list[front_pointer] != -1{
                front_pointer += 1;
            }

            while list[front_pointer] == -1 && front_pointer < list.len()-1 {
                replace.push(list[front_pointer]);
                front_pointer += 1;
            }

            if replace.len() >= replacement.len(){
                break;
            }
            replace = vec![];
        }

        front_pointer -= replace.len();

        // replace if room otherwise move on
        if replace.len() >= replacement.len() && !modified.contains(&origin){
            for i in front_pointer..front_pointer + replacement.len(){
                list[i] = replacement[0];
                back_pointer += 1;
                list[back_pointer] = -1;
            }
            if back_pointer > replacement.len(){
                back_pointer -= replacement.len();
            }
        }

        modified.insert(origin);
        replacement = vec![];
        replace = vec![];
    }

    checksum(list);
}

fn d9_1(test: String){
    let mut list: Vec<i64> = create_input(test);

    'outer: for i in 0..list.len()-1{
        if list[i] == -1{
            for j in (0..list.len()).rev(){
                if list[j] != -1{
                    list[i] = list[j].clone();
                    list[j] = -1;
                    break;
                }
                if j <= i{
                    break 'outer;
                }
            }
        }
    }

    checksum(list);
}

fn main() {
    let test = read_to_string("./input.txt").unwrap();
    // let test = "23331331214141314022333133121414131402";
    // d9_1(test.clone());
    d9_2(test.clone().parse().unwrap());
}
