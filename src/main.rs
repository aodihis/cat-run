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
    let player_ava: Texture2D = load_texture("assets/cat.png").await.unwrap();
    let mut game = Game::new(player_ava);
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
