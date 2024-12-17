use macroquad::prelude::*;
mod object;
use object::Object;

#[macroquad::main("MyGame")]
async fn main() {
    let mut player: Object = object::Object {
        loc: Vec2 {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        },
        color: GREEN,
    };

    loop {
        clear_background(RED);

        let key = get_keys_down();

        match key {
            key if key.contains(&KeyCode::W) => player.loc.y -= 1.0,
            key if key.contains(&KeyCode::S) => player.loc.y += 1.0,
            key if key.contains(&KeyCode::A) => player.loc.x -= 1.0,
            key if key.contains(&KeyCode::D) => player.loc.x += 1.0,
            key if key.contains(&KeyCode::Escape) => break,
            _ => (),
        }

        player.draw();

        draw_rectangle(0.0, 1.0, 120.0, 60.0, GREEN);
        draw_text("Hello, world!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
