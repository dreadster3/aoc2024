use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result: i32 = re
        .captures_iter(input)
        .map(|capture| capture.extract())
        .map(|(_, [a, b])| {
            let number_a = i32::from_str_radix(a, 10).unwrap();
            let number_b = i32::from_str_radix(b, 10).unwrap();
            number_a * number_b
        })
        .sum();

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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
