mod snake;

use std::process::exit;

use piston_window::*;

use snake::Direction;
use snake::Snake;

pub const BLOCK_SIZE: f64 = 25.0;
pub const BOARD_SIZE: (u16, u16) = (30, 30);

fn main() {
    let snake_color = [0.0, 0.5, 0.3, 1.0];
    let mut snake = Snake::new(snake_color, BLOCK_SIZE);

    let board_color = [0.5, 0.5, 0.5, 1.0];
    let (width, height) = BOARD_SIZE;
    let width_size = BLOCK_SIZE * (width as f64);
    let height_size = BLOCK_SIZE * (height as f64);

    let mut timer = 0.0;

    let mut window: PistonWindow = WindowSettings::new("Snake Game", [width_size, height_size])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear(board_color, g);
            snake.draw_snake(&c, g);
        });

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => snake.turn(Direction::Up),
                Key::S => snake.turn(Direction::Down),
                Key::A => snake.turn(Direction::Left),
                Key::D => snake.turn(Direction::Right),
                _ => (),
            }
        }

        e.update(|arg| {
            timer += arg.dt;
            if timer > 0.2 {
                snake.move_snake();
                if snake.check_collision(BOARD_SIZE, BLOCK_SIZE) {
                    println!("snake died");
                    exit(0);
                }
                timer = 0.0;
            }
        });
    }
}
