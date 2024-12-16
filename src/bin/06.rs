use std::{any::Any, borrow::BorrowMut, collections::VecDeque, fmt::Debug};

advent_of_code::solution!(6);

struct StaticQueue<T: Debug> {
    queue: VecDeque<T>,
    capacity: usize,
}

impl<T: Debug> StaticQueue<T> {
    fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn enqueue(&mut self, item: T) -> Option<T> {
        if self.queue.len() == self.capacity {
            let output = self.dequeue();
            self.enqueue(item);
            return output;
        }

        self.queue.push_back(item);
        None
    }

    fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn elements(&self) -> impl Iterator<Item = &T> + Clone + Debug {
        return self.queue.iter();
    }
}

trait Positional: Debug {
    fn position(&self) -> Position;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone, Copy)]
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

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

#[derive(Clone)]
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    fn length(&self) -> u32 {
        return ((self.start.x - self.end.x).abs() + (self.start.y - self.end.y).abs()) as u32;
    }

    fn is_horizontal(&self) -> bool {
        return self.start.y == self.end.y;
    }

    fn is_vertical(&self) -> bool {
        return self.start.x == self.end.x;
    }

    fn is_parallel(&self, other: &Line) -> bool {
        return (self.is_horizontal() && other.is_horizontal())
            || (self.is_vertical() && other.is_vertical());
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("line({:?} -> {:?})", self.start, self.end).as_str())
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

        let guard = self.guard_mut();
        let mut line_builder = vec![guard.position()];
        let mut history = StaticQueue::<Line>::new(3);
        let mut count = 0u32;
        while guard.position().is_within(start, end) {
            guard.walk();

            if objects
                .iter()
                .filter(|o| o.position().eq(&guard.position()))
                .count()
                > 0
            {
                println!("Turning around");
                let start = line_builder.first().unwrap().clone();
                let end = line_builder.last().unwrap().clone();
                history.enqueue(Line::new(start, end));
                line_builder.clear();
                line_builder.push(end);
                guard.back();
                guard.turn_right();
                guard.walk();
            }

            line_builder.push(guard.position());
            let elements = history.elements();
            println!("{elements:?}");
            if elements.clone().count() == 3 {
                let first = elements.clone().next().unwrap();
                let last = elements.clone().last().unwrap();

                if first.is_parallel(last) && first.length() == last.length() {
                    count += 1
                }
            }
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
