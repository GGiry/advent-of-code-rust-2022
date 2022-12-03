use std::cmp::max;

fn puzzle_input_to_list_of_bag_of_calories(puzzle_input: &str) -> Vec<Vec<u32>> {
    let input_lines = puzzle_input.lines();

    let mut result = vec![];
    let mut current_bag = vec![];

    for line in input_lines {
        if line.is_empty() {
            result.push(current_bag);
            current_bag = vec![];
        } else {
            current_bag.push(line.parse().unwrap())
        }
    }
    result.push(current_bag);

    result
}

fn max_calories(bag_list: Vec<Vec<u32>>) -> u32 {
    let mut result = 0;

    for bag in bag_list {
        let bag_value: u32 = bag.iter().sum();
        result = max(result, bag_value);
    }

    result
}

fn get_calories_of_bag_with_max_calories(puzzle_input: &str) -> u32 {
    max_calories(puzzle_input_to_list_of_bag_of_calories(puzzle_input))
}

fn sort_list_of_bag(mut list_of_bags: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    list_of_bags.sort_unstable_by(|a, b| {
        let a_sum: u32 = a.iter().sum();
        let b_sum: u32 = b.iter().sum();
        a_sum.cmp(&b_sum).reverse()
    });
    list_of_bags
}

fn get_calories_of_top3_bags(sorted_bag_list: Vec<Vec<u32>>) -> u32 {
    let mut result = 0;

    for bag in &sorted_bag_list[0..=2] {
        let bag_value: u32 = bag.iter().sum();
        result += bag_value;
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_calories_of_bag_with_max_calories(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let bag_list = puzzle_input_to_list_of_bag_of_calories(input);
    let sorted_bag_list = sort_list_of_bag(bag_list);
    Some(get_calories_of_top3_bags(sorted_bag_list))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }

    #[test]
    fn convert_calories_list_to_list_of_calories() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let expected_list_of_bags = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        let result = puzzle_input_to_list_of_bag_of_calories(input);

        assert_eq!(result, expected_list_of_bags);
    }

    #[test]
    fn compute_max_calories() {
        let calories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        let result = max_calories(calories);
        assert_eq!(result, 24000);
    }

    #[test]
    fn sort_of_vec_of_vec() {
        let calories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        let expected = vec![
            vec![7000, 8000, 9000],
            vec![5000, 6000],
            vec![10000],
            vec![1000, 2000, 3000],
            vec![4000],
        ];

        let result = sort_list_of_bag(calories);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_top3_sum() {
        let calories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        let result = get_calories_of_top3_bags(sort_list_of_bag(calories));
        assert_eq!(result, 45000);
    }
}
