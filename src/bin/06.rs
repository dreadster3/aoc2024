use std::{any::Any, borrow::BorrowMut, fmt::Debug};

advent_of_code::solution!(6);

trait Positional: Debug {
    fn position(&self) -> Position;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Clone, Copy)]
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

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
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

#[derive(Debug)]
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
    width: u32,
    height: u32,
    objects: Vec<Box<dyn Positional>>,
}

impl Map {
    fn play(&mut self) -> u32 {
        let objects = self.objects().map(|o| *o).collect::<Vec<Object>>();
        let width = self.width as u32;
        let height = self.height as u32;
        let start = Position { x: 0, y: 0 };
        let end = Position {
            x: width as i32 - 1,
            y: height as i32 - 1,
        };

        let guard = self.guard_mut();
        let mut history = vec![guard.position()];
        let mut count = 0;
        while guard.position().is_within(start, end) {
            println!("{guard:?}");
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

#[derive(Debug)]
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
        width: width as u32,
        height: height as u32,
        objects,
    };

    let count = map.play();

    Some(count)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
