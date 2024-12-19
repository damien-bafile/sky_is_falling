use macroquad::prelude::*;

mod object;
mod scene;
use crate::scene::Scene;
use object::*;

#[macroquad::main("MyGame")]
async fn main() {
    let mut scene: Scene = Scene::new();

    scene.add_object(Object {
        loc: Vec2 {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        },
        obj_type: ObjectType::Circle { radius: 10.0 },
        color: GREEN,
        obj_role: ObjectRol::Player,
    });

    scene.add_object(Object {
        loc: Vec2 { x: 100.0, y: 100.0 },
        obj_type: ObjectType::Rectangle {
            width: 20.0,
            height: 20.0,
        },
        color: BLACK,
        obj_role: ObjectRol::Enemy,
    });

    loop {
        clear_background(BLUE);

        let key = get_keys_down();

        if let Some(player) = scene.get_object_mut(ObjectRol::Player) {
            match key {
                key if key.contains(&KeyCode::W) => player.loc.y -= 1.0,
                key if key.contains(&KeyCode::S) => player.loc.y += 1.0,
                key if key.contains(&KeyCode::A) => player.loc.x -= 1.0,
                key if key.contains(&KeyCode::D) => player.loc.x += 1.0,
                key if key.contains(&KeyCode::Q) => break,
                key if key.contains(&KeyCode::Escape) => break,
                _ => (),
            }
        }

        scene.draw_all();

        draw_text("Hello, world!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
