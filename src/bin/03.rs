fn priority_of(item_type: char) -> usize {
    if ('a'..='z').contains(&item_type) {
        item_type as usize - 'a' as usize + 1
    } else {
        26 + item_type as usize - 'A' as usize + 1
    }
}

fn get_priority_of_item_common_in_both_compartments(rucksack: String) -> Result<u32, ()> {
    let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);

    for item_type in compartment_1.chars() {
        if compartment_2.contains(item_type) {
            return Ok(priority_of(item_type) as u32);
        }
    }

    Err(())
}

fn compute_sum_of_priorities(input: &str) -> u32 {
    let mut result = 0;

    for rucksack in input.lines() {
        result += get_priority_of_item_common_in_both_compartments(rucksack.to_string())
            .expect("Cannot find common item.");
    }

    result
}

fn get_priority_of_item_type_common_in_rucksacks(
    rucksack_1: &str,
    rucksack_2: &str,
    rucksack_3: &str,
) -> Result<u32, ()> {
    for item_type in rucksack_1.chars() {
        if rucksack_2.contains(item_type) && rucksack_3.contains(item_type) {
            return Ok(priority_of(item_type) as u32);
        }
    }

    Err(())
}

fn compute_sum_of_badges_priorities(input: &str) -> u32 {
    let mut result = 0;
    let mut input_lines = input.lines();

    loop {
        let rucksack_1 = input_lines.next();
        if rucksack_1.is_none() {
            break;
        }
        let rucksack_2 = input_lines.next().expect("Not enough rucksacks");
        let rucksack_3 = input_lines.next().expect("Not enough rucksacks");

        result += get_priority_of_item_type_common_in_rucksacks(
            rucksack_1.unwrap(),
            rucksack_2,
            rucksack_3,
        )
        .expect("Cannot find common item");
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(compute_sum_of_priorities(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(compute_sum_of_badges_priorities(input))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn can_find_common_item() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let result =
            get_priority_of_item_common_in_both_compartments(rucksack.to_string()).unwrap();
        assert_eq!(result, 16);
    }

    #[test]
    fn can_find_priority_of_a_letter() {
        assert_eq!(1, priority_of('a'));
        assert_eq!(27, priority_of('A'));
        assert_eq!(16, priority_of('p'));
        assert_eq!(42, priority_of('P'));
    }

    #[test]
    fn find_common_item_in_rucksacks() {
        let rucksack_1 = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack_2 = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let rucksack_3 = "PmmdzqPrVvPwwTWBwg";

        let result =
            get_priority_of_item_type_common_in_rucksacks(rucksack_1, rucksack_2, rucksack_3)
                .unwrap();

        assert_eq!(result, 18);
    }
}
