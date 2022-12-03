use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_rucksacks() -> Vec<String> {
    let file = File::open("./inputs/day_three.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
}

#[derive(Debug, Clone, Copy)]
struct Item(char);

impl From<Item> for u32 {
    fn from(item: Item) -> Self {
        let num = item.0 as u32;
        match item.0.is_ascii_lowercase() {
            true => num % 96,
            false => (num + 26) % 64,
        }
    }
}

#[allow(dead_code)]
fn part_one() -> i32 {
    let rucksacks = get_rucksacks();
    let mut sum = 0;
    for rucksack in rucksacks {
        let mapped: Vec<u32> = rucksack.chars().map(|car| u32::from(Item(car))).collect();
        let (first_compartment, second_compartment) = mapped.split_at(mapped.len() / 2);
        for element in first_compartment {
            if second_compartment.contains(element) {
                sum += *element as i32;
                break;
            }
        }
    }
    sum
}

#[allow(dead_code)]
fn part_two() -> i32 {
    let rucksacks: Vec<Vec<u32>> = get_rucksacks().iter()
        .map(|rucksack| rucksack.chars().map(|car| u32::from(Item(car))).collect()).collect();

    rucksacks
        .chunks_exact(3)
        .fold(0, |sum, chunk| {
            let mut badge_item = 0;
            for item in &chunk[0] {
                if chunk[1].contains(&item) && chunk[2].contains(&item) {
                    badge_item = *item;
                    break;
                }
            }
            sum + badge_item as i32
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 8233);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 2821);
    }
}
