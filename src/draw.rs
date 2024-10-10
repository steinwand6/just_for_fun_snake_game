use piston_window::*;

use crate::snake::Snake;

pub struct Window {
    window: PistonWindow,
    // width: u16,
    // height: u16,
    block_size: f64,
    background_color: [f32; 4],
}

impl Window {
    pub fn init_window(
        (width, height): (u16, u16),
        block_size: f64,
        background_color: [f32; 4],
    ) -> Window {
        let width_size = block_size * (width as f64);
        let height_size = block_size * (height as f64);
        Self {
            window: WindowSettings::new("Snake Game", [width_size, height_size])
                .exit_on_esc(true)
                .build()
                .unwrap(),
            // width,
            // height,
            block_size,
            background_color,
        }
    }

    pub fn next(&mut self) -> Option<Event> {
        self.window.next()
    }

    pub fn draw(&mut self, e: &Event, snake: &Snake) {
        self.draw_window(e);
        self.draw_snake(snake, e);
    }

    fn draw_window(&mut self, e: &Event) {
        self.window.draw_2d(e, |_, g, _| {
            clear(self.background_color, g);
        });
    }

    fn draw_snake(&mut self, snake: &Snake, e: &Event) {
        self.window.draw_2d(e, |c, g, _| {
            snake.get_body().iter().for_each(|(x, y)| {
                rectangle(
                    snake.get_color(),
                    [*x, *y, self.block_size, self.block_size], // rectangle
                    c.transform,
                    g,
                )
            })
        });
    }
}
