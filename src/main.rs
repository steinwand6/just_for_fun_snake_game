use std::collections::LinkedList;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [750; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // green of snake
    let snake_color = [0.0, 0.5, 0.3, 1.0];
    let mut snake = Snake::new(snake_color);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            snake.draw_snake(&c, g);
        });

        if let Event::Input(Input::Button(key), _) = e {
            match key.button {
                Button::Keyboard(Key::W) => snake.turn(Direction::Up),
                Button::Keyboard(Key::S) => snake.turn(Direction::Down),
                Button::Keyboard(Key::A) => snake.turn(Direction::Left),
                Button::Keyboard(Key::D) => snake.turn(Direction::Right),
                _ => (),
            }
        }
    }
}

struct Snake {
    color: [f32; 4],
    body: LinkedList<(f64, f64)>,
    tail: (f64, f64),
    direction: Direction,
}

impl Snake {
    pub fn new(color: [f32; 4]) -> Self {
        let mut body = LinkedList::new();
        body.push_back((150.0, 50.0));
        body.push_back((100.0, 50.0));
        body.push_back((50.0, 50.0));
        Self {
            color,
            body,
            tail: (50.0, 50.0),
            direction: Direction::Right,
        }
    }

    pub fn draw_snake(&self, c: &Context, g: &mut G2d) {
        self.body.iter().for_each(|(x, y)| {
            rectangle(
                self.color,
                [*x, *y, 50.0, 50.0], // rectangle
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
        self.move_snake();
    }

    pub fn move_snake(&mut self) {
        let head = self.body.pop_front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1 - 50.0),
            Direction::Down => (head.0, head.1 + 50.0),
            Direction::Left => (head.0 - 50.0, head.1),
            Direction::Right => (head.0 + 50.0, head.1),
        };
        self.body.push_front(head);
        self.body.push_front(new_head);
        self.tail = self.body.pop_back().unwrap();
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
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
