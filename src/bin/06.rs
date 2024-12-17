use std::{any::Any, collections::HashSet, fmt::Debug, ops::Add};

advent_of_code::solution!(6);

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Into<Vector2<i32>> for Direction {
    fn into(self) -> Vector2<i32> {
        match self {
            Direction::Right => Vector2 { x: 1, y: 0 },
            Direction::Left => Vector2 { x: -1, y: 0 },
            Direction::Up => Vector2 { x: 0, y: -1 },
            Direction::Down => Vector2 { x: 0, y: 1 },
        }
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Vector2<T> {
    x: T,
    y: T,
}

impl Add for Vector2<i32> {
    type Output = Vector2<i32>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

trait Positional {
    fn position(&self) -> Vector2<i32>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Clone)]
struct Obstacle {
    position: Vector2<i32>,
}

impl Positional for Obstacle {
    fn position(&self) -> Vector2<i32> {
        self.position.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
struct Guard {
    position: Vector2<i32>,
    direction: Direction,
}

impl Guard {
    fn next_position(&self) -> Vector2<i32> {
        self.position + self.direction.clone().into()
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn walk(&mut self) {
        self.position = self.next_position()
    }
}

impl Into<char> for Guard {
    fn into(self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Left => '<',
            Direction::Down => 'v',
        }
    }
}

impl Positional for Guard {
    fn position(&self) -> Vector2<i32> {
        self.position.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    guard: Guard,
    obstacles: Vec<Obstacle>,
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut representation = String::new();

        for i in 0..self.height {
            for j in 0..self.width {
                let position = Vector2 {
                    x: j as i32,
                    y: i as i32,
                };
                if self
                    .obstacles
                    .iter()
                    .filter(|o| o.position() == position)
                    .count()
                    > 0
                {
                    representation.push('#');
                } else if self.guard.position() == position {
                    representation.push(self.guard.clone().into());
                } else {
                    representation.push('.');
                }
            }
            representation.push('\n');
        }

        f.write_str(representation.as_str())
    }
}

impl Map {
    fn is_position_within(&self, position: Vector2<i32>) -> bool {
        let width = self.width as i32;
        let height = self.height as i32;

        if position.x < 0 || position.x >= width {
            return false;
        }

        if position.y < 0 || position.y >= height {
            return false;
        }

        true
    }

    fn simulation_loop<F>(&mut self, loop_function: &mut F)
    where
        F: FnMut(&Map, Vector2<i32>) -> Option<bool>,
    {
        loop {
            if !self.is_position_within(self.guard.position()) {
                break;
            }

            let mut next_position = self.guard.next_position();

            if self
                .obstacles
                .iter()
                .filter(|o| o.position() == next_position.clone())
                .count()
                > 0
            {
                self.guard.turn_right();
                next_position = self.guard.next_position();
            }

            if let Some(_) = loop_function(self, next_position) {
                break;
            };

            self.guard.walk();
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().chars().count();

    let height = input.lines().count();

    let objects = input
        .lines()
        .enumerate()
        .flat_map(move |(i, r)| {
            r.chars().enumerate().filter_map(move |(j, c)| {
                let position = Vector2::<i32> {
                    x: j as i32,
                    y: i as i32,
                };

                match c {
                    '#' => Some(Box::new(Obstacle { position }) as Box<dyn Positional>),
                    '^' => Some(Box::new(Guard {
                        direction: Direction::Up,
                        position,
                    }) as Box<dyn Positional>),
                    _ => None,
                }
            })
        })
        .collect::<Vec<_>>();

    let guard = objects
        .iter()
        .filter_map(|o| o.as_any().downcast_ref::<Guard>())
        .next()
        .unwrap();

    let obstacles = objects
        .iter()
        .filter_map(|o| o.as_any().downcast_ref::<Obstacle>())
        .map(|o| o.clone())
        .collect::<Vec<_>>();

    let mut map = Map {
        width,
        height,
        guard: guard.clone(),
        obstacles,
    };

    let mut visited = HashSet::new();

    map.simulation_loop(&mut |map, _| {
        visited.insert(map.guard.position());

        None
    });

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    let objects = input
        .lines()
        .enumerate()
        .flat_map(move |(i, r)| {
            r.chars().enumerate().filter_map(move |(j, c)| {
                let position = Vector2::<i32> {
                    x: j as i32,
                    y: i as i32,
                };

                match c {
                    '#' => Some(Box::new(Obstacle { position }) as Box<dyn Positional>),
                    '^' => Some(Box::new(Guard {
                        direction: Direction::Up,
                        position,
                    }) as Box<dyn Positional>),
                    _ => None,
                }
            })
        })
        .collect::<Vec<_>>();

    let guard = objects
        .iter()
        .filter_map(|o| o.as_any().downcast_ref::<Guard>())
        .next()
        .unwrap();

    let obstacles = objects
        .iter()
        .filter_map(|o| o.as_any().downcast_ref::<Obstacle>())
        .map(|o| o.clone())
        .collect::<Vec<_>>();

    let mut map = Map {
        width,
        height,
        guard: guard.clone(),
        obstacles,
    };

    let mut solutions = HashSet::new();
    let mut overall_visited = HashSet::new();
    overall_visited.insert((map.guard.position(), map.guard.direction.clone()));
    map.simulation_loop(&mut |map, next_position| {
        if overall_visited.contains(&(map.guard.position().clone(), map.guard.direction.clone())) {
            return None;
        }
        overall_visited.insert((map.guard.position().clone(), map.guard.direction.clone()));

        let mut temp_map = map.clone();
        let temp_obstacle = Obstacle {
            position: next_position.clone(),
        };
        temp_map.obstacles.push(temp_obstacle.clone());

        let mut visited_dir = HashSet::new();
        let mut is_loop = false;
        temp_map.simulation_loop(&mut |map, _| {
            if visited_dir.contains(&(map.guard.position.clone(), map.guard.direction.clone())) {
                is_loop = true;
                return Some(is_loop);
            }

            visited_dir.insert((map.guard.position().clone(), map.guard.direction.clone()));
            None
        });

        if is_loop {
            solutions.insert(temp_obstacle.position());
        }

        None
    });

    Some(solutions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
