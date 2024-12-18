use std::iter::repeat;

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
enum DiskEntry {
    Space,
    Occupied(u64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut iterator = vec![DiskEntry::Occupied(0), DiskEntry::Space]
        .into_iter()
        .cycle();

    let layout = input
        .chars()
        .filter_map(|c| usize::from_str_radix(&c.to_string(), 10).ok())
        .enumerate()
        .flat_map(|(i, times)| {
            match iterator.next() {
                Some(DiskEntry::Space) => repeat(DiskEntry::Space),
                Some(DiskEntry::Occupied(_)) => repeat(DiskEntry::Occupied((i / 2) as u64)),
                _ => panic!(),
            }
            .take(times)
        })
        .collect::<Vec<_>>();

    let occupied = layout
        .iter()
        .filter(|d| matches!(d, DiskEntry::Occupied(_)))
        .count();
    let mut back_iter = layout.clone().into_iter().rev();
    let mut front_iter = layout.clone().into_iter();
    let mut organized = vec![];

    while organized.len() < occupied {
        let next = match front_iter.next() {
            Some(DiskEntry::Occupied(i)) => DiskEntry::Occupied(i),
            Some(DiskEntry::Space) => back_iter.find(|x| !matches!(x, DiskEntry::Space)).unwrap(),
            _ => panic!(),
        };

        organized.push(next);
    }

    let result: u64 = organized
        .iter()
        .enumerate()
        .filter_map(|(i, e)| match e {
            DiskEntry::Space => None,
            DiskEntry::Occupied(id) => Some(*id * i as u64),
        })
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
