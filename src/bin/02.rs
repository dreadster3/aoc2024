use std::fmt::Debug;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum DiffType {
    Increase,
    Decrease,
    Invalid,
}

fn diff_type(a: u32, b: u32) -> DiffType {
    let diff = (b as i32) - (a as i32);
    if diff < 0 && diff.abs() <= 3 {
        return DiffType::Decrease;
    }
    if diff > 0 && diff.abs() <= 3 {
        return DiffType::Increase;
    }

    return DiffType::Invalid;
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| u32::from_str_radix(n, 10).unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|v| v.len() > 1);

    let result = lines.filter(|v| is_valid(v.clone())).count();

    Some(result as u32)
}

fn diff_types<'a>(v: &'a Vec<u32>) -> impl Iterator<Item = DiffType> + 'a + Clone + Debug {
    v.iter()
        .zip(v.iter().skip(1).chain(std::iter::once(&0)))
        .take(v.len() - 1)
        .map(|(&a, &b)| diff_type(a, b))
}

fn is_valid(v: Vec<u32>) -> bool {
    let diff_types = diff_types(&v);

    diff_types.clone().all(|t| matches!(t, DiffType::Increase))
        || diff_types.clone().all(|t| matches!(t, DiffType::Decrease))
}

fn smallest_diff(iterator: impl Iterator<Item = DiffType> + Clone) -> (usize, DiffType) {
    let diff1 = iterator
        .clone()
        .filter(|t| matches!(t, DiffType::Decrease))
        .count()
        .abs_diff(iterator.clone().count());
    let diff2 = iterator
        .clone()
        .filter(|t| matches!(t, DiffType::Increase))
        .count()
        .abs_diff(iterator.clone().count());

    if diff1 < diff2 {
        return (diff1, DiffType::Decrease);
    }

    (diff2, DiffType::Increase)
}

fn is_fixable(v: Vec<u32>) -> bool {
    let diff_types_iter = diff_types(&v);
    let (distance, dtype) = smallest_diff(diff_types_iter.clone());

    if distance > 2 {
        return false;
    }

    let to_remove_index = diff_types_iter.clone().position(|t| !(t == dtype)).unwrap();

    let without = v
        .iter()
        .take(to_remove_index)
        .chain(v.iter().skip(to_remove_index + 1))
        .map(|&v| v)
        .collect::<Vec<u32>>();

    let other_without = v
        .iter()
        .take(to_remove_index + 1)
        .chain(v.iter().skip(to_remove_index + 2))
        .map(|&v| v)
        .collect::<Vec<u32>>();

    is_valid(without) || is_valid(other_without)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| u32::from_str_radix(n, 10).unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|v| v.len() > 1);

    let result = lines.clone().filter(|v| is_valid(v.clone())).count();
    let fixable = lines
        .clone()
        .filter(|v| !is_valid(v.clone()))
        .filter(|v| is_fixable(v.clone()))
        .count();

    Some((result + fixable) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
