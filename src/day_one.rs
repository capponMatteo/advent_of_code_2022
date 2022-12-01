use std::{fs::File, io::BufRead, io::BufReader};

pub fn get_lines() -> Vec<String> {
    let file = File::open("./inputs/day_one.txt").unwrap();
    let mut lines = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    lines.push(String::from(""));
    lines
}

pub fn part_one() -> i32 {
    let input = get_lines();

    let mut most_calories = i32::MIN;
    let mut sum = 0;

    for line in input.iter() {
        if line.is_empty() {
            if sum > most_calories {
                most_calories = sum;
            }
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }
    most_calories
}

pub fn part_two() -> i32 {
    let input = get_lines();

    let mut podium = [0, 0, 0];
    let mut sum = 0;

    'outer: for line in input.iter() {
        let Ok(calories) = line.parse::<i32>() else {
            for i in 0..=2 {
                if sum > podium[i] {
                    for j in (i + 1..=2).rev() {
                        podium[j] = podium[j - 1];
                    }
                    podium[i] = sum;
                    sum = 0;
                    continue 'outer;
                }
            }
            sum = 0;
            continue;
        };
        sum += calories;
    }
    podium.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 68442);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 204837);
    }
}
