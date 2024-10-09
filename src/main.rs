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

        /* if let Event::Input(Input::Button(key), _) = e {
            match key.button {
                Button::Keyboard(Key::W) => y -= 50.0,
                Button::Keyboard(Key::A) => x -= 50.0,
                Button::Keyboard(Key::D) => x += 50.0,
                Button::Keyboard(Key::S) => y += 50.0,
                _ => (),
            }
        } */
    }
}

struct Snake {
    color: [f32; 4],
    body: LinkedList<(f64, f64)>,
    tail: (f64, f64),
}

impl Snake {
    pub fn new(color: [f32; 4]) -> Self {
        let mut body = LinkedList::new();
        body.push_back((150.0, 50.0));
        body.push_back((100.0, 50.0));
        body.push_back((50.0, 50.0));
        Self {
            color,
            body: body,
            tail: (50.0, 50.0),
        }
    }

    fn draw_snake(&self, c: &Context, g: &mut G2d) {
        self.body.iter().for_each(|(x, y)| {
            rectangle(
                self.color,
                [*x, *y, 50.0, 50.0], // rectangle
                c.transform,
                g,
            )
        });
    }
}
