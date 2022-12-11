use crate::utils::{read_lines, build_input_path};

const ITEMS_TYPES: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Item(char);

impl Item {
    fn new(letter: char) -> Self {
        Self {
            0: letter,
        }
    }
    fn get_priority(&self) -> u32 {
        ITEMS_TYPES.find(self.0).unwrap() as u32 + 1
    }
}

#[derive(Clone)]
struct Rucksack {
    compartment1: Vec<Item>,
    compartment2: Vec<Item>,
}

impl Rucksack {
    fn new(items: &str) -> Self {
        let half_len = items.len() / 2;
        assert_eq!(items.len(), half_len * 2);
        Self {
            compartment1: items.chars().take(half_len).map(|letter| Item::new(letter)).collect::<Vec<Item>>(),
            compartment2: items.chars().skip(half_len).map(|letter| Item::new(letter)).collect::<Vec<Item>>(),
        }
    }
    fn find_item_in_both_compartments(&self) -> Option<Item> {
        for item in self.compartment1.iter() {
            if self.compartment2.contains(item) {
                return Some(*item);
            }
        }
        None
    }
    fn contains_item(&self, item: Item) -> bool {
        self.compartment1.contains(&item) || self.compartment2.contains(&item)
    }
}

struct RucksackGroup([Rucksack; 3]);

impl RucksackGroup {
    fn new(first: Rucksack, second: Rucksack, third: Rucksack) -> Self {
        Self {
            0: [first, second, third],
        }
    }
    fn find_common_item_type(&self) -> Option<Item> {
        let rucksacks = &self.0;
        for c in ITEMS_TYPES.chars() {
            let item = Item(c);
            if rucksacks.iter().all(|rucksack| rucksack.contains_item(item)) {
                return Some(item);
            }
        }
        None
    }
}

pub fn part1() {
    let path = build_input_path(3, "input1.txt");
    let lines = read_lines(path.as_str());

    let mut sum = 0;

    for line in lines.iter() {
        let rucksack = Rucksack::new(line);
        sum += rucksack.find_item_in_both_compartments().unwrap().get_priority();
    }

    println!("priority sum: {}", sum);
}

pub fn part2() {
    let path = build_input_path(3, "input1.txt");
    let lines = read_lines(path.as_str());

    let mut sum = 0;

    let rucksacks = lines.iter().map(|line| Rucksack::new(line)).collect::<Vec<Rucksack>>();
    for i in (0..rucksacks.len()).step_by(3) {
        let rucksack_group = RucksackGroup::new(rucksacks[i].clone(), rucksacks[i + 1].clone(), rucksacks[i + 2].clone());
        let item = rucksack_group.find_common_item_type().unwrap();
        sum += item.get_priority();
    }

    println!("priority sum: {}", sum);
}