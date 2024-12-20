use macroquad::ui::InputHandler;

use crate::states::GameState;

pub struct MenuState;

impl MenuState {
    pub fn new() -> Self {
        Self
    }
}

impl GameState for MenuState {
    fn update(&mut self) {
        // Update the game state
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Enter) {
            // Transition to the next state
            println!("Starting the game");
        }
    }

    fn draw(&self) {
        // Draw the game state
        macroquad::text::draw_text(
            "Press Enter to start the game",
            100.0,
            200.0,
            40.0,
            macroquad::color::WHITE,
        );
    }

    fn next(&self) -> Option<Box<dyn GameState>> {
        // Return the next game state
        if InputHandler::is_key_pressed(macroquad::input::KeyCode::Enter) {
            Some(Box::new(GameState::new()))
        } else {
            None
        }
    }
}
