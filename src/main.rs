mod draw;
mod snake;

use std::process::exit;

use piston_window::*;

use draw::Window;
use snake::Direction;
use snake::Snake;

pub const BLOCK_SIZE: f64 = 25.0;
pub const BOARD_SIZE: (u16, u16) = (30, 30);

fn main() {
    let snake_color = [0.0, 0.5, 0.3, 1.0];
    let mut snake = Snake::new(snake_color, BLOCK_SIZE);

    let board_color = [0.5, 0.5, 0.5, 1.0];

    let mut timer = 0.0;

    let mut window = Window::init_window(BOARD_SIZE, BLOCK_SIZE, board_color);

    while let Some(e) = window.next() {
        window.draw(&e, &snake);

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
