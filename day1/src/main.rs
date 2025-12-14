use std::fs;

fn main() {
    let filename = "src/data.txt";
    let file_contents = get_file(filename);
    part1(&file_contents);
    part2(&file_contents);
}

fn part1(file_contents: &str) {
    println!("Part 1:");
    let file = file_contents.lines();
    let mut index = 50;
    let mut count = 0;
    for line in file {
        let step: i32 = line[1..].parse().unwrap();
        let direction: char = line.chars().next().unwrap();
        let new_index = turn(index, &direction, step, 100 as i32);
        if new_index == 0 {
            count += 1
        }
        index = new_index;
    }
    println!("{}", count);
}

fn part2(file_contents: &str) {
    println!("Part 2:");
    let file = file_contents.lines();
    let mut index = 50;
    let mut count = 0;
    let size = 100;
    for line in file {
        let step: i32 = line[1..].parse().unwrap();
        let passes = step / size;
        let actual_step = step % size;
        let direction: char = line.chars().next().unwrap();
        let new_index = turn(index, &direction, actual_step, size);
        count += passes;
        if new_index == 0
            || direction == 'R' && new_index < index && index != 0
            || direction == 'L' && new_index > index && index != 0
        {
            count += 1;
        }
        index = new_index;
    }
    println!("{}", count);
}

fn turn(index: i32, direction: &char, step: i32, size: i32) -> i32 {
    if *direction == 'R' {
        cw(index, step, size)
    } else {
        ccw(index, step, size)
    }
}

fn cw(index: i32, step: i32, size: i32) -> i32 {
    (index + step + size) % size
}

fn ccw(index: i32, step: i32, size: i32) -> i32 {
    (index - step + size) % size
}

fn get_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn get_direction(direction: &str) -> i32 {
    match direction {
        "L" => -1,
        "R" => 1,
        _ => 0,
    }
}
