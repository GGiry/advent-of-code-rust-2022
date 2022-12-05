use regex::Regex;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
struct Stack {
    crates: Vec<char>,
}

fn get_initial_state(input: &str) -> Vec<Stack> {
    let stack_count = input.lines().last().unwrap().split_whitespace().count();

    let mut result = vec![Stack { crates: vec![] }; stack_count];

    for line in input.lines().rev() {
        let mut chars = line.chars();

        let mut stack_index = 0;

        loop {
            chars.next();
            let item = chars.next();
            chars.next();
            chars.next();

            match item {
                None => break,
                Some(crate_item) => match crate_item {
                    ' ' => {
                        stack_index += 1;
                        continue;
                    }
                    _ => result[stack_index].crates.push(crate_item),
                },
            }

            stack_index += 1;
        }
    }

    result
}

fn split_initial_state_and_moves(input: &str) -> (String, String) {
    let mut initial_state = String::new();
    let mut moves = String::new();

    let mut move_flag = false;

    for line in input.lines() {
        if line.is_empty() {
            move_flag = true;
            continue;
        }

        if !move_flag {
            initial_state = initial_state.add(line).add("\n");
        } else {
            moves = moves.add(line).add("\n");
        }
    }

    (initial_state, moves)
}

fn perform_move_with_crane_9000(stacks: &mut [Stack], count: u32, from: usize, to: usize) {
    for _ in 0..count {
        let item = stacks[from].crates.pop().unwrap();
        stacks[to].crates.push(item);
    }
}

fn perform_move_with_crane_9001(stacks: &mut [Stack], count: u32, from: usize, to: usize) {
    let mut tmp: Vec<char> = vec![];

    for _ in 0..count {
        tmp.push(stacks[from].crates.pop().unwrap());
    }

    for item in tmp.iter().rev() {
        stacks[to].crates.push(*item);
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (initial_state, moves) = split_initial_state_and_moves(input);
    let mut stacks = get_initial_state(initial_state.as_str());

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for action in moves.lines() {
        for cap in re.captures_iter(action) {
            perform_move_with_crane_9000(
                &mut stacks,
                cap[1].parse().unwrap(),
                cap[2].parse::<usize>().unwrap() - 1,
                cap[3].parse::<usize>().unwrap() - 1,
            )
        }
    }

    let mut result = String::new();

    for stack in stacks {
        result.push(*stack.crates.last().unwrap());
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (initial_state, moves) = split_initial_state_and_moves(input);
    let mut stacks = get_initial_state(initial_state.as_str());

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for action in moves.lines() {
        for cap in re.captures_iter(action) {
            perform_move_with_crane_9001(
                &mut stacks,
                cap[1].parse().unwrap(),
                cap[2].parse::<usize>().unwrap() - 1,
                cap[3].parse::<usize>().unwrap() - 1,
            )
        }
    }

    let mut result = String::new();

    for stack in stacks {
        result.push(*stack.crates.last().unwrap());
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }

    #[test]
    fn initial_state() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
";
        let expected = vec![
            Stack {
                crates: vec!['1', 'Z', 'N'],
            },
            Stack {
                crates: vec!['2', 'M', 'C', 'D'],
            },
            Stack {
                crates: vec!['3', 'P'],
            },
        ];
        assert_eq!(expected, get_initial_state(input));
    }

    #[test]
    fn test_initial_state_and_move_separation() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
";
        let expected = (
            "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
"
            .to_string(),
            "move 1 from 2 to 1
"
            .to_string(),
        );

        assert_eq!(expected, split_initial_state_and_moves(input));
    }
}
