use std::{any::Any, collections::HashSet, fmt::Debug, iter::once};

advent_of_code::solution!(6);

trait Positional: Debug {
    fn position(&self) -> Position;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn up(&mut self) {
        self.y -= 1
    }

    fn down(&mut self) {
        self.y += 1
    }

    fn right(&mut self) {
        self.x += 1
    }

    fn left(&mut self) {
        self.x -= 1
    }

    fn is_within(&self, start: Position, end: Position) -> bool {
        return self.x >= start.x && self.x <= end.x && self.y >= start.y && self.y <= end.y;
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("position")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
struct Object {
    position: Position,
}

impl Positional for Object {
    fn position(&self) -> Position {
        self.position
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
    position: Position,
    direction: Direction,
}

impl Guard {
    fn walk(&mut self) {
        match self.direction {
            Direction::Right => self.position.right(),
            Direction::Left => self.position.left(),
            Direction::Up => self.position.up(),
            Direction::Down => self.position.down(),
        }
    }

    fn next_position(&self) -> Position {
        let mut current = self.position();
        match self.direction {
            Direction::Right => current.right(),
            Direction::Left => current.left(),
            Direction::Up => current.up(),
            Direction::Down => current.down(),
        };

        current
    }

    fn back(&mut self) {
        match self.direction {
            Direction::Right => self.position.left(),
            Direction::Left => self.position.right(),
            Direction::Up => self.position.down(),
            Direction::Down => self.position.up(),
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }
}

impl Positional for Guard {
    fn position(&self) -> Position {
        self.position
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct Map {
    width: usize,
    height: usize,
    objects: Vec<Box<dyn Positional>>,
}

impl Map {
    fn count_possible_loops(&mut self) -> u32 {
        let objects = self.objects().map(|o| *o).collect::<Vec<Object>>();
        let width = self.width;
        let height = self.height;
        let start = Position { x: 0, y: 0 };
        let end = Position {
            x: width as i32 - 1,
            y: height as i32 - 1,
        };

        let mut already_checked = HashSet::new();
        let guard = self.guard_mut();
        let mut count = 0;
        while guard.position().is_within(start, end) {
            if objects
                .iter()
                .filter(|o| o.position().eq(&guard.next_position()))
                .count()
                > 0
            {
                guard.turn_right();
            }
            guard.walk();

            let temp_obstable = Object {
                position: guard.next_position(),
            };
            let temp_guard = guard.clone();
            let temp_objects = objects
                .iter()
                .map(|&o| Box::new(o) as Box<dyn Positional>)
                .chain(once(Box::new(temp_guard) as Box<dyn Positional>))
                .chain(once(Box::new(temp_obstable) as Box<dyn Positional>))
                .collect::<Vec<_>>();

            let mut temp_map = Map {
                width,
                height,
                objects: temp_objects,
            };

            if !already_checked.contains(&temp_obstable.position()) && temp_map.is_loop() {
                count += 1;
            }
            already_checked.insert(temp_obstable.position());
        }

        count
    }

    fn count_unique_positions(&mut self) -> u32 {
        let objects = self.objects().map(|o| *o).collect::<Vec<Object>>();
        let width = self.width;
        let height = self.height;
        let start = Position { x: 0, y: 0 };
        let end = Position {
            x: width as i32 - 1,
            y: height as i32 - 1,
        };

        let guard = self.guard_mut();
        let mut history = vec![guard.position()];
        let mut count = 0;
        while guard.position().is_within(start, end) {
            guard.walk();

            if objects
                .iter()
                .filter(|o| o.position().eq(&guard.position()))
                .count()
                > 0
            {
                guard.back();
                guard.turn_right();
                guard.walk();
            }

            if !history.contains(&guard.position()) {
                count += 1;
                history.push(guard.position());
            }
        }

        count
    }

    fn is_loop(&mut self) -> bool {
        let objects = self.objects().map(|o| *o).collect::<Vec<Object>>();
        let mut visited = HashSet::new();

        while self.is_guard_within_map() {
            let guard = self.guard_mut();
            if objects
                .iter()
                .filter(|o| o.position().eq(&guard.next_position()))
                .count()
                > 0
            {
                guard.turn_right();
            }
            guard.walk();

            if visited.contains(&(guard.position(), guard.direction.clone())) {
                return true;
            }

            visited.insert((guard.position(), guard.direction.clone()));
        }

        false
    }

    fn is_guard_within_map(&self) -> bool {
        let start = Position { x: 0, y: 0 };
        let guard = self.guard();
        let end = Position {
            x: self.width as i32 - 1,
            y: self.height as i32 - 1,
        };

        guard.position().is_within(start, end)
    }

    fn guard(&self) -> &Guard {
        let guard = self
            .objects
            .iter()
            .filter_map(|o| o.as_any().downcast_ref::<Guard>())
            .next()
            .unwrap();

        return guard;
    }
    fn guard_mut(&mut self) -> &mut Guard {
        let guard = self
            .objects
            .iter_mut()
            .filter_map(|o| o.as_any_mut().downcast_mut::<Guard>())
            .next()
            .unwrap();

        return guard;
    }

    fn objects(&self) -> impl Iterator<Item = &Object> + Clone {
        let objects = self
            .objects
            .iter()
            .filter_map(|o| o.as_any().downcast_ref::<Object>());

        return objects;
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    let objects = input
        .lines()
        .enumerate()
        .flat_map(move |(i, r)| {
            r.chars().enumerate().filter_map(move |(j, c)| {
                let position = Position {
                    x: j as i32,
                    y: i as i32,
                };
                match c {
                    '#' => Some(Box::new(Object { position }) as Box<dyn Positional>),
                    '^' => Some(Box::new(Guard {
                        direction: Direction::Up,
                        position,
                    }) as Box<dyn Positional>),
                    _ => None,
                }
            })
        })
        .collect::<Vec<Box<dyn Positional>>>();

    let mut map = Map {
        width,
        height,
        objects,
    };

    let count = map.count_unique_positions();

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    let objects = input
        .lines()
        .enumerate()
        .flat_map(move |(i, r)| {
            r.chars().enumerate().filter_map(move |(j, c)| {
                let position = Position {
                    x: j as i32,
                    y: i as i32,
                };
                match c {
                    '#' => Some(Box::new(Object { position }) as Box<dyn Positional>),
                    '^' => Some(Box::new(Guard {
                        direction: Direction::Up,
                        position,
                    }) as Box<dyn Positional>),
                    _ => None,
                }
            })
        })
        .collect::<Vec<Box<dyn Positional>>>();

    let mut map = Map {
        width,
        height,
        objects,
    };

    let count = map.count_possible_loops();

    Some(count)
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
