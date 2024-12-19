pub mod game_over;
pub mod gameplay;
pub mod menu;

pub use game_over::GameOverState;
pub use gameplay::GameplayState;
pub use menu::MenuState;

pub trait State {
    fn update(&mut self, dt: f32);
    fn draw(&self);
    fn handle_input(&mut self, input: Input);
    fn next(&self) -> Option<Box<dyn State>>;
}
