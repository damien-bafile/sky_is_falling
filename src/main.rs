mod game;

#[macroquad::main("sky is falling")]
async fn main() {
    let mut game = game::Game::new();
    game.run().await;
}
