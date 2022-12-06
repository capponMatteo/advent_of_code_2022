use std::collections::HashSet;

fn get_datastream() -> Vec<u8> {
    let content = std::fs::read_to_string("./inputs/day_six.txt").unwrap();
    content.trim_end().to_owned().into_bytes()
}

fn solve(data_stream: Vec<u8>, windows_size: usize) -> Option<usize> {
    data_stream
        .windows(windows_size)
        .position(|window| {
            let mut hashset = HashSet::new();
            window.iter().for_each(|el| {
                hashset.insert(el);
            });
            hashset.len() == windows_size
        })
        .map(|val| val + windows_size)
}

#[allow(dead_code)]
fn part_one() -> Option<usize> {
    let data_stream = get_datastream();
    solve(data_stream, 4)
}

#[allow(dead_code)]
fn part_two() -> Option<usize> {
    let data_stream = get_datastream();
    solve(data_stream, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), Some(1723));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), Some(3708));
    }
}
