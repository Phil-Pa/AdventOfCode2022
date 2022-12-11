use std::collections::VecDeque;

use crate::utils::{read_lines, build_input_path};

static mut BUFFER: [u32; 4] = [0, 0, 0, 0];

fn letters_are_different(letters: &VecDeque<char>) -> bool {
    for (i, letter) in letters.iter().enumerate() {
        let number = *letter as u32;
        unsafe {
            BUFFER[i] = number;
        }
    }

    unsafe {
        BUFFER.sort();
        BUFFER[0] != BUFFER[1] && BUFFER[1] != BUFFER[2] && BUFFER[2] != BUFFER[3]
    }
}

fn letters_are_different2(letters: &VecDeque<char>, buffer: &mut Vec<char>) -> bool {
    buffer.clear();
    for letter in letters.iter() {
        if buffer.contains(letter) {
            return false;
        }
        buffer.push(*letter);
    }

    true
}

pub fn part1() {

    let path = build_input_path(6, "input1.txt");
    let lines = read_lines(path.as_str());

    let line = &lines[0];

    let mut letters = VecDeque::new();

    for (i, c) in line.chars().enumerate() {
        letters.push_front(c);

        if i < 3 {
            continue;
        }

        if letters_are_different(&letters) {
            let i = i + 1;
            println!("{}, {}", i, &line[i..(i + 4)]);
            return;
        }

        letters.pop_back();
    }

}

pub fn part2() {

    let path = build_input_path(6, "input1.txt");
    let lines = read_lines(path.as_str());

    let line = &lines[0];

    let mut letters = VecDeque::new();
    let mut buffer = Vec::with_capacity(14);

    for (i, c) in line.chars().enumerate() {
        letters.push_front(c);

        if i < 13 {
            continue;
        }

        if letters_are_different2(&letters, &mut buffer) {
            let i = i + 1;
            println!("{}, {}", i, &line[i..(i + 4)]);
            return;
        }

        letters.pop_back();
    }

}