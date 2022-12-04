use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Assignment {
    from: u32,
    to: u32,
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from_str, to_str) = s.split_once('-').ok_or("Cannot convert to Assignment")?;
        let from = from_str.parse().unwrap();
        let to = to_str.parse().unwrap();
        Ok(Assignment { from, to })
    }
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        if self.from <= other.from && self.to >= other.to {
            return true;
        }
        false
    }

    fn overlap(&self, other: &Assignment) -> bool {
        if (self.from..=self.to).contains(&other.from) || (self.from..=self.to).contains(&other.to)
        {
            return true;
        }
        false
    }
}

fn convert_line_to_assignments(line: &str) -> Result<(Assignment, Assignment), String> {
    let assignments = line.split_once(',').ok_or("Cannot find delimiter ','")?;

    Ok((
        Assignment::from_str(assignments.0).expect("Cannot convert first assignment"),
        Assignment::from_str(assignments.1).expect("Cannot convert second assignment"),
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    for line in input.lines() {
        let (elf_1_assignment, elf_2_assignment) = convert_line_to_assignments(line).unwrap();
        if elf_1_assignment.contains(&elf_2_assignment)
            || elf_2_assignment.contains(&elf_1_assignment)
        {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    for line in input.lines() {
        let (elf_1_assignment, elf_2_assignment) = convert_line_to_assignments(line).unwrap();
        if elf_1_assignment.overlap(&elf_2_assignment)
            || elf_2_assignment.overlap(&elf_1_assignment)
        {
            result += 1;
        }
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }

    #[test]
    fn test_from_str_for_assignment() {
        let assignment = "2-4";
        assert_eq!(
            Assignment { from: 2, to: 4 },
            Assignment::from_str(assignment).unwrap()
        );
    }

    #[test]
    fn can_convert_string_into_assignments() {
        let assignments = "2-4,6-8";
        assert_eq!(
            (Assignment { from: 2, to: 4 }, Assignment { from: 6, to: 8 }),
            convert_line_to_assignments(assignments).unwrap(),
        );
    }

    #[test]
    fn test_assignments_contains() {
        assert!(Assignment { from: 1, to: 5 }.contains(&Assignment { from: 2, to: 3 }));
        assert!(!(Assignment { from: 2, to: 3 }.contains(&Assignment { from: 1, to: 5 })));
    }

    #[test]
    fn test_assignments_overlap() {
        assert!(!(Assignment { from: 2, to: 4 }.overlap(&Assignment { from: 6, to: 8 })));
        assert!(Assignment { from: 5, to: 7 }.overlap(&Assignment { from: 7, to: 9 }));
    }
}
