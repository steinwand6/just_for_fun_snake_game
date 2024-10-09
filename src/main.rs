use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [500; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // green of snake
    let snake_color = [0.0, 0.5, 0.3, 1.0];

    let mut x = 50.0;
    let mut y = 50.0;
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle(
                snake_color,
                [x, y, 50.0, 50.0], // rectangle
                c.transform,
                g,
            );
        });

        if let Event::Input(Input::Button(key), _) = e {
            match key.button {
                Button::Keyboard(Key::W) => y -= 50.0,
                Button::Keyboard(Key::A) => x -= 50.0,
                Button::Keyboard(Key::D) => x += 50.0,
                Button::Keyboard(Key::S) => y += 50.0,
                _ => (),
            }
        }
    }
}
