use crate::game::{Game, GameResult, GameState};
use macroquad::prelude::*;

mod constant;
mod game;
mod helpers;


#[macroquad::main("Cat Run")]
async fn main() {
    let player_ava = load_texture("assets/cat.png").await.unwrap_or_else(|err| {
        eprintln!("ERROR loading player texture: {err}");
        Texture2D::empty()
    });

    let enemy_ava: Texture2D = load_texture("assets/enemy.png").await.unwrap_or_else(|err| {
        eprintln!("Error loading enemy texture: {err}");
        Texture2D::empty()
    });

    let mut game = Game::new(player_ava, enemy_ava);
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
            GameState::GameOver(GameResult::Win) => {
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
            GameState::GameOver(GameResult::Lose) => {
                game.draw();
                draw_rectangle(
                    0., 0., screen_width(), screen_height(), Color::new(0., 0., 0., 0.5)
                );

                draw_text("You Lose!", screen_width() / 2. - 100., screen_height() / 2. - 100., 30., BLACK);
                draw_text("Press enter to restart!", screen_width() / 2. - 100., screen_height() / 2. + 100., 30., BLACK);
                if is_key_pressed(KeyCode::Enter) {
                    game.restart();
                }
            },
        }
        next_frame().await;
    }
}
