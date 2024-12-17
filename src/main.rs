use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let mut pos = (screen_width() / 2.0, screen_height() / 2.0);
    loop {
        clear_background(RED);

        let key = get_keys_down();

        match key {
            key if key.contains(&KeyCode::W) => pos.1 -= 1.0,
            key if key.contains(&KeyCode::S) => pos.1 += 1.0,
            key if key.contains(&KeyCode::A) => pos.0 -= 1.0,
            key if key.contains(&KeyCode::D) => pos.0 += 1.0,
            key if key.contains(&KeyCode::Escape) => break,
            _ => (),
        }

        draw_circle(screen_width() / 2.0 - 15.0, 30.0, 30.0, BLUE);
        draw_rectangle(pos.0, pos.1, 120.0, 60.0, GREEN);
        draw_text("Hello, world!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
