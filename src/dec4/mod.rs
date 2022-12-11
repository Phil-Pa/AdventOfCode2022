use crate::utils::{read_lines, build_input_path};

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
        }
    }
    fn from_str(range: &str) -> Self {
        let parts = range.split('-').collect::<Vec<&str>>();
        let start = parts[0].parse().unwrap();
        let end = parts[1].parse().unwrap();
        Self::new(start, end)
    }
    fn fully_contains_other_range(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    fn overlapse_with(&self, other: &Range) -> bool {
        for i in self.start..=self.end {
            for j in other.start..=other.end {
                if i == j {
                    return true;
                }
            }
        }
        false
    }
}

pub fn part1() {
    let path = build_input_path(4, "input1.txt");
    let lines = read_lines(path.as_str());

    let mut count = 0;

    for line in lines.iter() {
        let ranges = line.split(',').collect::<Vec<&str>>();
        let range1 = Range::from_str(ranges[0]);
        let range2 = Range::from_str(ranges[1]);

        if range1.fully_contains_other_range(&range2) || range2.fully_contains_other_range(&range1) {
            count += 1;
        }
    }

    println!("count: {}", count);
}

pub fn part2() {
    let path = build_input_path(4, "input1.txt");
    let lines = read_lines(path.as_str());

    let mut count = 0;

    for line in lines.iter() {
        let ranges = line.split(',').collect::<Vec<&str>>();
        let range1 = Range::from_str(ranges[0]);
        let range2 = Range::from_str(ranges[1]);

        if range1.overlapse_with(&range2) {
            count += 1;
        }
    }

    println!("count: {}", count);
}