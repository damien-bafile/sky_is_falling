use crate::states::GameState;
use macroquad::prelude::clear_background;

pub struct GameOverState;

impl GameOverState {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameState for GameOverState {
    fn update(&mut self) {
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Enter) {
            println!("Returning to Menu!");
        }
    }

    fn draw(&self) {
        clear_background(macroquad::color::DARKGRAY);

        // Draw game over text
        macroquad::text::draw_text(
            "Game Over! Press Enter to Return to Menu",
            100.0,
            200.0,
            40.0,
            macroquad::color::WHITE,
        );
    }

    fn next(&self) -> Option<Box<dyn GameState>> {
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Enter) {
            Some(Box::new(crate::states::MenuState::new()))
        } else {
            None
        }
    }
}
