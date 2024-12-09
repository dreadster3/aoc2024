use std::{iter::zip, ops::Mul};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();

    input.lines().for_each(|line| {
        let words: Vec<&str> = line.split_whitespace().collect();
        left.push(u32::from_str_radix(words[0], 10).unwrap());
        right.push(u32::from_str_radix(words[1], 10).unwrap());
    });

    left.sort();
    right.sort();

    let zipped = zip(left, right);
    let differences = zipped.map(|(a, b)| a.abs_diff(b));
    let sum = differences.sum::<u32>();

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();

    input.lines().for_each(|line| {
        let words: Vec<&str> = line.split_whitespace().collect();
        left.push(u32::from_str_radix(words[0], 10).unwrap());
        right.push(u32::from_str_radix(words[1], 10).unwrap());
    });

    left.sort();
    right.sort();

    let result = left
        .iter()
        .map(|l| {
            let count = right
                .iter()
                .skip_while(|r| **r < *l)
                .take_while(|r| **r == *l)
                .count();

            *l * count as u32
        })
        .sum();

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
