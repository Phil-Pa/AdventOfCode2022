use crate::utils::{read_lines, build_input_path};

const LOST_POINTS: u32 = 0;
const DRAW_POINTS: u32 = 3;
const WIN_POINTS: u32 = 6;

fn get_points_for_pick(pick: char) -> u32 {
    match pick {
        // rock
        'X' => 1,
        // paper
        'Y' => 2,
        // scissor
        'Z' => 3,
        _ => panic!("unknown pick"),
    }
}

fn get_points_for_game1(my_pick: char, other_pick: char) -> u32 {
    let mut points = 0;

    points += get_points_for_pick(my_pick);

    match my_pick {
        'X' => {
            match other_pick {
                'A' => points += DRAW_POINTS,
                'B' => points += LOST_POINTS,
                'C' => points += WIN_POINTS,
                _ => panic!("unknown other pick"),
            }
        },
        'Y' => {
            match other_pick {
                'A' => points += WIN_POINTS,
                'B' => points += DRAW_POINTS,
                'C' => points += LOST_POINTS,
                _ => panic!("unknown other pick"),
            }
        },
        'Z' => {
            match other_pick {
                'A' => points += LOST_POINTS,
                'B' => points += WIN_POINTS,
                'C' => points += DRAW_POINTS,
                _ => panic!("unknown other pick"),
            }
        },
        _ => panic!("unknown pick"),
    }

    points
}

fn get_points_for_game2(should_pick: char, other_pick: char) -> u32 {
    let mut points = 0;

    let my_pick = match should_pick {
        'X' => {
            match other_pick {
                'A' => 'Z',
                'B' => 'X',
                'C' => 'Y',
                _ => panic!("unknown other pick"),
            }
        },
        'Y' => {
            match other_pick {
                'A' => 'X',
                'B' => 'Y',
                'C' => 'Z',
                _ => panic!("unknown other pick"),
            }
        },
        'Z' => {
            match other_pick {
                'A' => 'Y',
                'B' => 'Z',
                'C' => 'X',
                _ => panic!("unknown other pick"),
            }
        }
        _ => panic!("unknown pick"),
    };

    points += get_points_for_pick(my_pick);

    match my_pick {
        'X' => {
            match other_pick {
                'A' => points += DRAW_POINTS,
                'B' => points += LOST_POINTS,
                'C' => points += WIN_POINTS,
                _ => panic!("unknown other pick"),
            }
        },
        'Y' => {
            match other_pick {
                'A' => points += WIN_POINTS,
                'B' => points += DRAW_POINTS,
                'C' => points += LOST_POINTS,
                _ => panic!("unknown other pick"),
            }
        },
        'Z' => {
            match other_pick {
                'A' => points += LOST_POINTS,
                'B' => points += WIN_POINTS,
                'C' => points += DRAW_POINTS,
                _ => panic!("unknown other pick"),
            }
        },
        _ => panic!("unknown pick"),
    }

    points
}

pub fn part1() {

    let path = build_input_path(2, "input1.txt");
    let lines = read_lines(path.as_str());

    let my_pick_options = ['X', 'Y', 'Z'];
    let other_pick_options = ['A', 'B', 'C'];

    let mut points = 0;

    for line in lines.iter() {
        let mut my_pick = 'X';
        let mut other_pick = 'A';

        line.split(' ').for_each(|pick| {
            let pick = pick.chars().next().unwrap();
            if my_pick_options.contains(&pick) {
                my_pick = pick;
            } else if other_pick_options.contains(&pick) {
                other_pick = pick;
            } else {
                panic!("unknown pick option");
            }
        });

        points += get_points_for_game1(my_pick, other_pick);
    }

    println!("points: {}", points);
}

pub fn part2() {

    let path = build_input_path(2, "input1.txt");
    let lines = read_lines(path.as_str());

    let my_pick_options = ['X', 'Y', 'Z'];
    let other_pick_options = ['A', 'B', 'C'];

    let mut points = 0;

    for line in lines.iter() {
        let mut my_pick = 'X';
        let mut other_pick = 'A';

        line.split(' ').for_each(|pick| {
            let pick = pick.chars().next().unwrap();
            if my_pick_options.contains(&pick) {
                my_pick = pick;
            } else if other_pick_options.contains(&pick) {
                other_pick = pick;
            } else {
                panic!("unknown pick option");
            }
        });

        points += get_points_for_game2(my_pick, other_pick);
    }

    println!("points: {}", points);
}