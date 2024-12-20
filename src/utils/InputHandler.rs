use macroquad::input::{is_key_down, is_key_pressed, KeyCode};

pub struct InputHandler;

impl InputHandler {
    pub fn is_key_held(key: KeyCode) -> bool {
        is_key_down(key)
    }

    pub fn is_key_pressed(key: KeyCode) -> bool {
        is_key_pressed(key)
    }
}
