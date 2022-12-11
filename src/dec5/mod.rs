use std::collections::VecDeque;

use crate::utils::{read_lines, build_input_path};

fn print_stacks(stacks: &Vec<VecDeque<char>>) {
    for stack in stacks.iter() {
        println!("{:?}", stack);
    }
}

pub fn part1() {
    let path = build_input_path(5, "input1.txt");
    let lines = read_lines(path.as_str());

    let mut stack_description_lines = lines.iter().take_while(|line| !line.is_empty()).collect::<Vec<_>>();
    let move_lines = lines.iter().skip(stack_description_lines.len() + 1).collect::<Vec<_>>();

    let number_line = stack_description_lines.iter().find(|line| line.contains('1')).unwrap();
    let mut number_indices = Vec::<usize>::new();

    // get number indices
    for (i, c) in number_line.chars().enumerate() {
        if c.is_digit(10) {
            number_indices.push(i);
        }
    }

    // remove number line
    stack_description_lines.remove(stack_description_lines.len() - 1);

    let mut stacks = Vec::<VecDeque<char>>::with_capacity(number_indices.len());

    for number_index in number_indices.iter() {
        let mut stack = VecDeque::new();
        for line in stack_description_lines.iter().rev() {
            let letter = line.chars().nth(*number_index).unwrap();
            if letter != ' ' {
                stack.push_front(letter);
            }
        }
        stacks.push(stack);
    }

    print_stacks(&stacks);
    println!();

    for line in move_lines.iter() {
        let parts = line.split(' ').collect::<Vec<_>>();

        let mut num_moves = parts[1].parse::<u32>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();

        while num_moves > 0 && stacks[from - 1].len() > 0 {
            let letter = stacks[from - 1].pop_front().unwrap();
            stacks[to - 1].push_front(letter);
            num_moves -= 1;
            print_stacks(&stacks);
            println!();
        }
    }

    for stack in stacks.iter() {
        if let Some(letter) = stack.front() {
            println!("top: {}", letter);
        }
    }
}

pub fn part2() {
    let path = build_input_path(5, "input1.txt");
    let lines = read_lines(path.as_str());

    let mut stack_description_lines = lines.iter().take_while(|line| !line.is_empty()).collect::<Vec<_>>();
    let move_lines = lines.iter().skip(stack_description_lines.len() + 1).collect::<Vec<_>>();

    let number_line = stack_description_lines.iter().find(|line| line.contains('1')).unwrap();
    let mut number_indices = Vec::<usize>::new();

    // get number indices
    for (i, c) in number_line.chars().enumerate() {
        if c.is_digit(10) {
            number_indices.push(i);
        }
    }

    // remove number line
    stack_description_lines.remove(stack_description_lines.len() - 1);

    let mut stacks = Vec::<VecDeque<char>>::with_capacity(number_indices.len());

    for number_index in number_indices.iter() {
        let mut stack = VecDeque::new();
        for line in stack_description_lines.iter().rev() {
            let letter = line.chars().nth(*number_index).unwrap();
            if letter != ' ' {
                stack.push_front(letter);
            }
        }
        stacks.push(stack);
    }

    print_stacks(&stacks);
    println!();

    let mut letters = Vec::<char>::new();

    for line in move_lines.iter() {
        let parts = line.split(' ').collect::<Vec<_>>();

        let mut num_moves = parts[1].parse::<u32>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();

        letters.clear();

        while num_moves > 0 && stacks[from - 1].len() > 0 {
            let letter = stacks[from - 1].pop_front().unwrap();
            num_moves -= 1;
            letters.push(letter);
            print_stacks(&stacks);
            println!();
        }

        for letter in letters.iter().rev() {
            stacks[to - 1].push_front(*letter);
        }
    }

    for stack in stacks.iter() {
        if let Some(letter) = stack.front() {
            println!("top: {}", letter);
        }
    }
}