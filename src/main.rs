use crate::game::Game;
use macroquad::prelude::*;

mod constant;
mod game;
mod helpers;

enum GameState {
    Menu,
    Playing,
    GameOver,
}
#[macroquad::main("Cat")]
async fn main() {
    let mut game = Game::new();
    loop {
        // set_camera(&game.cam);
        clear_background(WHITE);

        match game.state {
            GameState::Menu => {
                draw_text("Press enter to start!", 100., 100., 42., BLACK);
                if is_key_pressed(KeyCode::Enter) {
                    game.state = GameState::Playing;
                }
            },
            GameState::Playing => {
                game.draw();
            },
            GameState::GameOver => {},
        }
        next_frame().await;
    }
}
