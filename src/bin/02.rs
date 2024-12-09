advent_of_code::solution!(2);

#[derive(Debug)]
enum DiffType {
    Increase,
    Decrease,
    Invalid,
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

    let result = lines
        .map(|v| {
            let diff_types = v
                .iter()
                .zip(v.iter().skip(1).chain(std::iter::once(&0)))
                .take(v.len() - 1)
                .map(|(&a, &b)| {
                    let diff = (b as i32) - (a as i32);
                    if diff < 0 && diff.abs() <= 3 {
                        return DiffType::Decrease;
                    }
                    if diff > 0 && diff.abs() <= 3 {
                        return DiffType::Increase;
                    }

                    return DiffType::Invalid;
                });

            diff_types.clone().all(|t| matches!(t, DiffType::Increase))
                || diff_types.clone().all(|t| matches!(t, DiffType::Decrease))
        })
        .filter(|&b| b)
        .count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
