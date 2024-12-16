use std::fs::read_to_string;

fn create_input(test: String) -> Vec<DiskEntry> {
    let mut memory = Vec::new();

    for (idx, size) in test.trim().char_indices() {
        let size = size.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            let id = idx / 2;
            memory.push(DiskEntry::File { id, size });
        } else {
            memory.push(DiskEntry::FreeSpace { size });
        }
    }

    memory
}

fn checksum(memory: &[DiskEntry]) {
    let mut position = 0;
    let mut result = 0;

    for block in memory {
        match *block {
            DiskEntry::FreeSpace { size } => position += size,
            DiskEntry::File { id, size } => {
                for _ in 0..size {
                    result += id * position;
                    position += 1;
                }
            }
        }
    }

    println!("{}", result);
}

fn d9_1(test: String){
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

    let mut sum = 0;

    for i in 0..list.len(){
        if list[i] != -1 {
            sum += list[i] * i as i64;
        }
    }

    println!("{}", sum);
}

enum DiskEntry {
    FreeSpace { size: usize },
    File { id: usize, size: usize },
}

fn d9_2(mut memory: Vec<DiskEntry>) {
    let mut i = memory.len() - 1;
    while i > 0 {
        if let DiskEntry::File { id, size: filesize } = memory[i] {
            let mut insertion_idx = 0;

            loop {
                if let DiskEntry::FreeSpace { size: freesize } = memory[insertion_idx] {
                    if freesize > filesize {
                        memory[i] = DiskEntry::FreeSpace { size: filesize };
                        memory[insertion_idx] = DiskEntry::File { id, size: filesize };
                        memory.insert(
                            insertion_idx + 1,
                            DiskEntry::FreeSpace {
                                size: freesize - filesize,
                            },
                        );
                        break;
                    }

                    if freesize == filesize {
                        memory[i] = DiskEntry::FreeSpace { size: filesize };
                        memory[insertion_idx] = DiskEntry::File { id, size: filesize };
                        break;
                    }
                }

                if insertion_idx == i {
                    break;
                }

                insertion_idx += 1;
            }
        }
        i -= 1;
    }

    checksum(&memory);
}

fn main() {
    let test = read_to_string("./input.txt").unwrap();
    // let test = "23331331214141314022333133121414131402";
    let list = create_input(test);
    // d9_1(test.clone());
    d9_2(list);
}
