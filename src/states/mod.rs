pub mod game_over;
pub mod gameplay;
pub mod menu;

pub use game_over::GameOverState;
pub use gameplay::GameplayState;
pub use menu::MenuState;

pub trait GameState {
    fn update(&mut self);
    fn draw(&self);
    fn next(&self) -> Option<Box<dyn GameState>>;
}
