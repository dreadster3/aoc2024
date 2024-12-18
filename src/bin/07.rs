advent_of_code::solution!(7);

fn join_operator(lhs: u64, rhs: u64) -> u64 {
    let mut append_result = String::new();
    append_result.push_str(lhs.to_string().as_str());
    append_result.push_str(rhs.to_string().as_str());
    return u64::from_str_radix(append_result.as_str(), 10).unwrap();
}

fn all_possible_results(current: Option<u64>, numbers: Vec<u64>) -> Vec<u64> {
    let next_element = numbers[0];
    if numbers.len() == 1 {
        return vec![
            (current.unwrap_or(0) + next_element),
            (current.unwrap_or(1) * next_element),
        ];
    }

    match current {
        None => all_possible_results(Some(next_element), numbers[1..].to_vec()),
        Some(value) => {
            let mut results =
                all_possible_results(Some(value + next_element), numbers[1..].to_vec());
            results.append(&mut all_possible_results(
                Some(value * next_element),
                numbers[1..].to_vec(),
            ));

            results
        }
    }
}

fn all_possible_results_part_two(current: Option<u64>, numbers: Vec<u64>) -> Vec<u64> {
    let next_element = numbers[0];
    if numbers.len() == 1 {
        return vec![
            (current.unwrap_or(0) + next_element),
            (current.unwrap_or(1) * next_element),
            (join_operator(current.unwrap_or(0), next_element)),
        ];
    }

    match current {
        None => all_possible_results_part_two(Some(next_element), numbers[1..].to_vec()),
        Some(value) => {
            let mut results =
                all_possible_results_part_two(Some(value + next_element), numbers[1..].to_vec());
            results.append(&mut all_possible_results_part_two(
                Some(value * next_element),
                numbers[1..].to_vec(),
            ));
            results.append(&mut all_possible_results_part_two(
                Some(join_operator(value, next_element)),
                numbers[1..].to_vec(),
            ));

            results
        }
    }
}

fn is_valid(result: u64, numbers: Vec<u64>) -> bool {
    let all_results = all_possible_results(None, numbers);

    all_results.iter().find(|&r| r.eq(&result)).is_some()
}

fn is_valid_part_two(result: u64, numbers: Vec<u64>) -> bool {
    let all_results = all_possible_results_part_two(None, numbers);

    all_results.iter().find(|&r| r.eq(&result)).is_some()
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|l| {
            let (result_str, numbers_str) = l.split_once(":").unwrap();
            let result = u64::from_str_radix(result_str.trim(), 10).unwrap();
            let numbers = numbers_str
                .trim()
                .split(" ")
                .map(|n| u64::from_str_radix(n.trim(), 10).unwrap())
                .collect::<Vec<_>>();

            (result, numbers)
        })
        .filter(|(result, numbers)| is_valid(*result, numbers.clone()))
        .map(|(result, _)| result)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|l| {
            let (result_str, numbers_str) = l.split_once(":").unwrap();
            let result = u64::from_str_radix(result_str.trim(), 10).unwrap();
            let numbers = numbers_str
                .trim()
                .split(" ")
                .map(|n| u64::from_str_radix(n.trim(), 10).unwrap())
                .collect::<Vec<_>>();

            (result, numbers)
        })
        .filter(|(result, numbers)| is_valid_part_two(*result, numbers.clone()))
        .map(|(result, _)| result)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
