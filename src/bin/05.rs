use std::collections::HashMap;

advent_of_code::solution!(5);

enum Compare {
    Higher,
    Equal,
    Lower,
}

fn compare(order: HashMap<u32, Vec<u32>>, a: u32, b: u32) -> Option<bool> {
    if let Some(values) = order.get(&a) {
        if values.contains(&b) {
            return Some(true);
        }
    }

    if let Some(values) = order.get(&b) {
        if values.contains(&a) {
            return Some(false);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let order = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|s| (s.split("|").next().unwrap(), s.split("|").last().unwrap()))
        .map(|(k, v)| {
            (
                u32::from_str_radix(k, 10).unwrap(),
                u32::from_str_radix(v, 10).unwrap(),
            )
        })
        .fold(HashMap::<u32, Vec<u32>>::new(), |mut acc, (k, v)| {
            acc.entry(k)
                .and_modify(|vec| vec.push(v))
                .or_insert(vec![v]);
            acc
        });

    println!("{:?}", order);

    let result: u32 = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| l.split(",").map(|n| u32::from_str_radix(n, 10).unwrap()))
        .filter(|v| {
            v.clone()
                .is_sorted_by(|&a, &b| compare(order.clone(), a, b).unwrap())
        })
        .map(|v| v.clone().nth(v.clone().count() / 2).unwrap())
        .sum();

    Some(result)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
