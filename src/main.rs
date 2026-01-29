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
                game.update(get_frame_time());
                game.draw();
            },
            GameState::GameOver => {
                game.draw();
                draw_rectangle(
                    0., 0., screen_width(), screen_height(), Color::new(0., 0., 0., 0.5)
                );

                draw_text("You Win!", screen_width() / 2. - 100., screen_height() / 2. - 100., 30., BLACK);
                draw_text("Press enter to restart!", screen_width() / 2. - 100., screen_height() / 2. + 100., 30., BLACK);
                if is_key_pressed(KeyCode::Enter) {
                    game.restart();
                }
            },
        }
        next_frame().await;
    }
}
