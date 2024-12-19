use crate::states::{GameOverState, GameState, GameplayState, MenuState};

pub struct Game {
    current_state: Box<dyn GameState>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            current_state: Box::new(MenuState::new()),
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.current_state.update();
            self.current_state.draw();

            // Handle transitions
            if let Some(next_state) = self.current_state.next_state() {
                self.current_state = next_state;
            }

            // Break if exiting
            if self.current_state.should_exit() {
                break;
            }

            // Control frame rate
            macroquad::window::next_frame().await;
        }
    }
}
