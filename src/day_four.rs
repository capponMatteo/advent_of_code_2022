use std::{fs::File, io::BufRead, io::BufReader, ops::RangeInclusive};

#[allow(dead_code)]
fn get_assignments() -> std::io::Result<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let file = File::open("./inputs/day_four.txt")?;
    BufReader::new(file)
        .lines()
        .map(|l| {
            l.map(|line| {
                let pairs = line.split_once(',').expect("comma in the middle!");
                let first = pairs.0.split_once('-').expect("hyphen in first pair!");
                let second = pairs.1.split_once('-').expect("hyphen in second pair!");
                (
                    (first.0.parse::<u32>().unwrap()..=first.1.parse::<u32>().unwrap()),
                    (second.0.parse::<u32>().unwrap()..=second.1.parse::<u32>().unwrap()),
                )
            })
        })
        .collect()
}

#[allow(dead_code)]
fn part_one() -> i32 {
    get_assignments().unwrap().iter().fold(0, |sum, pair| {
        if pair.1.clone().all(|id| pair.0.contains(&id))
            || pair.0.clone().all(|id| pair.1.contains(&id))
        {
            sum + 1
        } else {
            sum
        }
    })
}

#[allow(dead_code)]
fn part_two() -> i32 {
    get_assignments().unwrap().iter().fold(0, |sum, pair| {
        if pair.1.clone().any(|id| pair.0.contains(&id))
            || pair.0.clone().any(|id| pair.1.contains(&id))
        {
            sum + 1
        } else {
            sum
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 515);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 883);
    }
}
