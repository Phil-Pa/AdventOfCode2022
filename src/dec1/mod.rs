use crate::utils::{read_lines, build_input_path};

pub fn part1() {

    let path = build_input_path(1, "input1.txt");
    let lines = read_lines(path.as_str());
    
    let mut sums = vec![];
    let mut sum = 0;

    for line in lines.iter() {
        if let Ok(number) = line.parse::<u32>() {
            sum += number;
        } else {
            sums.push(sum);
            sum = 0;
        }
    }

    println!("{}", sums.iter().max().unwrap());
}

pub fn part2() {

    let path = build_input_path(1, "input1.txt");

    let lines = read_lines(path.as_str());
    let mut sums = vec![];
    let mut sum = 0;
    
    for line in lines.iter() {
        if let Ok(number) = line.parse::<u32>() {
            sum += number;
        } else {
            sums.push(sum);
            sum = 0;
        }
    }

    sums.sort();

    let sum = sums.iter().rev().take(3).sum::<u32>();

    println!("{}", sum);
}