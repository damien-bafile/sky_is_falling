use crate::states::GameState;
use macroquad::prelude::clear_background;

pub struct GameplayState {
    player_x: f32,
    player_y: f32,
}

impl GameplayState {
    pub fn new() -> Self {
        Self {
            player_x: 200.0,
            player_y: 200.0,
        }
    }
}

impl GameState for GameplayState {
    fn update(&mut self) {
        // Move the player with arrow keys
        if macroquad::input::is_key_down(macroquad::input::KeyCode::Right) {
            self.player_x += 5.0;
        }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::Left) {
            self.player_x -= 5.0;
        }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::Up) {
            self.player_y -= 5.0;
        }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::Down) {
            self.player_y += 5.0;
        }

        // Exit to Game Over state if the player presses Escape
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Escape) {
            println!("Game Over!");
        }
    }

    fn draw(&self) {
        clear_background(macroquad::color::BLACK);

        // Draw the player
        macroquad::shapes::draw_circle(self.player_x, self.player_y, 20.0, macroquad::color::BLUE);

        // Draw instructions
        macroquad::text::draw_text(
            "Use Arrow Keys to Move, Press Escape to End Game",
            50.0,
            50.0,
            30.0,
            macroquad::color::WHITE,
        );
    }

    fn next(&self) -> Option<Box<dyn GameState>> {
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Escape) {
            Some(Box::new(crate::states::GameOverState::new()))
        } else {
            None
        }
    }
}
