use std::collections::LinkedList;

use piston_window::*;

pub struct Snake {
    color: [f32; 4],
    body: LinkedList<(f64, f64)>,
    direction: Direction,
    block_size: f64,
}

impl Snake {
    pub fn new(color: [f32; 4], block_size: f64) -> Self {
        let mut body = LinkedList::new();
        body.push_back((block_size * 4.0, block_size));
        body.push_back((block_size * 3.0, block_size));
        body.push_back((block_size * 2.0, block_size));
        Self {
            color,
            body,
            direction: Direction::Right,
            block_size,
        }
    }

    pub fn draw_snake(&self, c: &Context, g: &mut G2d) {
        self.body.iter().for_each(|(x, y)| {
            rectangle(
                self.color,
                [*x, *y, self.block_size, self.block_size], // rectangle
                c.transform,
                g,
            )
        });
    }

    pub fn turn(&mut self, direction: Direction) {
        println!("{:?} {:?} -> {:?}", self.body, self.direction, direction);
        if direction == Direction::opposite(&self.direction)
        /* || direction == self.direction */
        {
            return;
        }
        self.direction = direction;
    }

    pub fn move_snake(&mut self) {
        let head = self.body.pop_front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1 - self.block_size),
            Direction::Down => (head.0, head.1 + self.block_size),
            Direction::Left => (head.0 - self.block_size, head.1),
            Direction::Right => (head.0 + self.block_size, head.1),
        };
        self.body.push_front(head);
        self.body.push_front(new_head);
        self.body.pop_back();
    }

    pub fn check_collision(&self, (max_x, max_y): (u16, u16), block_size: f64) -> bool {
        let (max_x, max_y) = (max_x as f64 * block_size, max_y as f64 * block_size);
        println!(
            "x: {}, y:{}, max_x: {}, max_y: {}",
            self.body.front().unwrap().0,
            self.body.front().unwrap().1,
            max_x,
            max_y
        );
        self.body
            .iter()
            .any(|(x, y)| *x < 0.0 || *y < 0.0 || *x >= max_x || *y >= max_y)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}
