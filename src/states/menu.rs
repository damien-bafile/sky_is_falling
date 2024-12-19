use crate::states::GameState;

pub struct MenuState;

impl GameState for MenuState {
    fn update(&mut self, _dt: f32) {
        // Update the game state
    }

    fn draw(&self) {
        // Draw the game state
    }

    fn handle_input(&mut self, _input: Input) {
        // Handle input
    }

    fn next(&self) -> Option<Box<dyn GameState>> {
        // Return the next game state
        None
    }
}
