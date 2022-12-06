use std::collections::VecDeque;

const FILE_NAME: &str = "./inputs/day_five.txt";

fn get_stacks() -> std::io::Result<Vec<VecDeque<char>>> {
    let bytes: Vec<u8> = std::fs::read(FILE_NAME).unwrap().into_iter().filter(|byte| *byte != b'\r').collect();
    // The number of stacks is determined by summing one to the index of the first \n and dividing it by 4
    let num_stacks = (bytes.iter().position(|byte| *byte == b'\n').unwrap() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(num_stacks);
    let mut index = 0x1; // First char
    (0..num_stacks).for_each(|_| stacks.push(VecDeque::new()));
    loop {
        if bytes[index] == b' ' {
            // A char every 0x4 bytes
            index += 0x4;
            continue;
        }
        // Normalize the index from file offset to array index
        stacks[(index / 4) % num_stacks].push_back(bytes[index] as char);
        // A char every 0x4 bytes
        index += 0x4;
        // We're done when we reach the line with stacks' numbers
        if bytes[index] == b'1' {
            break;
        }
    }
    Ok(stacks)
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn get_moves() -> std::io::Result<Vec<Move>> {
    let content = std::fs::read_to_string(FILE_NAME).unwrap();
    let moves = content
        .lines()
        .filter(|line| line.starts_with("move"))
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            Move {
                amount: words[1].parse().unwrap(),
                from: words[3].parse::<usize>().unwrap() - 1,
                to: words[5].parse::<usize>().unwrap() - 1,
            }
        })
        .collect();
    Ok(moves)
}

#[allow(dead_code)]
fn part_one() -> String {
    let mut stacks = get_stacks().unwrap();
    let moves = get_moves().unwrap();
    for mov in moves {
        for _ in 0..mov.amount {
            let to_move = stacks[mov.from].pop_front().unwrap();
            stacks[mov.to].push_front(to_move)
        }
    }
    let mut ret = String::new();
    for stack in stacks {
        ret.push(*stack.front().unwrap());
    }
    ret
}

#[allow(dead_code)]
fn part_two() -> String {
    let mut stacks = get_stacks().unwrap();
    let moves = get_moves().unwrap();
    let mut temp = VecDeque::new();
    for mov in moves {
        for _ in 0..mov.amount {
            let to_move = stacks[mov.from].pop_front().unwrap();
            temp.push_back(to_move);
        }
        while let Some(el) = temp.pop_back() {
            stacks[mov.to].push_front(el);
        }
        temp.clear();
    }
    let mut ret = String::new();
    for stack in stacks {
        ret.push(*stack.front().unwrap());
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), "SHMSDGZVC");
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), "VRZGHDFBQ");
    }
}
