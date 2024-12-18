use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

advent_of_code::solution!(8);

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Debug for Vec2<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({:?}, {:?})", self.x, self.y).as_str())
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec2<T>;
    fn mul(self, scalar: T) -> Self::Output {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

type Point = Vec2<i32>;
impl Point {
    fn antinode(self, other: Self) -> (Point, Point) {
        let distance_vector = self - other;

        (other + (distance_vector * -1), self + distance_vector)
    }
}

#[derive(Debug, Clone, Copy)]
struct Tower {
    position: Point,
    frequency: char,
}

impl Tower {
    fn antinode(&self, other: Self) -> (Point, Point) {
        let distance_vector = self.position - other.position;

        (
            other.position + (distance_vector * -1),
            self.position + distance_vector,
        )
    }
}

fn is_within_map(height: usize, width: usize, point: Vec2<i32>) -> bool {
    if point.x < 0 || point.x >= width as i32 {
        return false;
    }
    if point.y < 0 || point.y >= height as i32 {
        return false;
    }

    true
}

fn exponential_antinodes(points: Vec<Point>) -> Vec<Point> {
    let first = points[0];
    if points.len() == 2 {
        let (antinode_1, antinode_2) = first.antinode(points[1]);
        return vec![antinode_1, antinode_2];
    }

    let others = points[1..].to_vec();

    let mut antinodes = others
        .iter()
        .flat_map(|&t| {
            let (antinode_1, antinode_2) = first.antinode(t);
            vec![antinode_1, antinode_2]
        })
        .collect::<Vec<_>>();
    antinodes.append(&mut exponential_antinodes(others));

    antinodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let towers = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars().enumerate().filter_map(move |(j, f)| match f {
                '.' => None,
                _ => Some((f, Point::new(j as i32, i as i32))),
            })
        })
        .fold(HashMap::<char, Vec<Point>>::new(), |mut acc, (f, p)| {
            acc.entry(f).and_modify(|v| v.push(p)).or_insert(vec![p]);
            acc
        });

    let mut antinodes = HashSet::new();
    for (_, v) in towers {
        antinodes.extend(exponential_antinodes(v));
    }

    let result = antinodes
        .iter()
        .filter(|&a| is_within_map(height, width, *a))
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
