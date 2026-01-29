use crate::constant::MAP_SIZE;
use crate::helpers::draw_helpers::{draw_ava, draw_exit, draw_wall};
use crate::helpers::pathfinding::find_path;
use macroquad::input::is_key_down;
use macroquad::prelude::{KeyCode, Texture2D};
use macroquad::window::{screen_height, screen_width};

pub enum GameResult {
    Win,
    Lose,
}
pub enum GameState {
    Menu,
    Playing,
    GameOver(GameResult),
}

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
    Exit,
}

struct Enemy {
    x: usize,
    y: usize,
    timer: f32
}

pub struct Game {
    pub state: GameState,
    map: [[Tile; MAP_SIZE]; MAP_SIZE],
    player_ava: Texture2D,
    px: usize,
    py: usize,
    player_timer: f32,
    player_delay: f32,
    enemy_ava: Texture2D,
    enemies: Vec<Enemy>
}

impl Game {
    pub fn new(player_ava: Texture2D, enemy_ava: Texture2D) -> Self {
        let mut map : [[Tile; MAP_SIZE]; MAP_SIZE] = [[Tile::Floor; MAP_SIZE]; MAP_SIZE];
        for i in 0..MAP_SIZE {
            map[i][MAP_SIZE - 1] = Tile::Wall;
            map[i][0] = Tile::Wall;
        }

        for i in 0..MAP_SIZE {
            map[MAP_SIZE - 1][i] = Tile::Wall;
            map[0][i] = Tile::Wall;
        }

        map[MAP_SIZE - 2][MAP_SIZE - 2] = Tile::Exit;
        map[15][15] = Tile::Wall;
        map[9][10] = Tile::Wall;
        let mut enemies = vec![];
        enemies.push(Enemy {
            x: 12,
            y: 12,
            timer: 0.
        });

        enemies.push(Enemy {
            x: 5,
            y: 7,
            timer: 0.
        });

        enemies.push(Enemy {
            x: 9,
            y: 13,
            timer: 0.
        });
        Self {
            map,
            state: GameState::Menu,
            player_ava,
            px: 2,
            py: 10,
            player_timer: 0.,
            player_delay: 0.15,
            enemy_ava,
            enemies
        }
    }

    pub fn restart(&mut self) {
        self.px = 2;
        self.py = 10;
        self.state = GameState::Menu;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.player_timer -= delta_time;
        let mut py = self.py;
        let mut px = self.px;

        let enemies_pos: Vec<_> = self
            .enemies
            .iter()
            .map(|m| (m.x, m.y))
            .collect();

        if self.player_timer <= 0. {
            if is_key_down(KeyCode::Up){
                py -= 1;
            }
            if is_key_down(KeyCode::Down){
                py += 1;
            }
            if is_key_down(KeyCode::Left){
                px -= 1;
            }
            if is_key_down(KeyCode::Right){
                px += 1;
            }
            if !(self.px == px && self.py == py) {
                if let Some(row) = self.map.get(py) {
                    if let Some(tile) = row.get(px) {
                        if *tile == Tile::Floor || *tile == Tile::Exit{
                            self.px = px;
                            self.py = py;
                            self.player_timer = self.player_delay;
                        }
                        if *tile == Tile::Exit {
                            self.state = GameState::GameOver(GameResult::Lose);
                            return;
                        }
                        if enemies_pos.contains(&(px, py)) {
                            self.state = GameState::GameOver(GameResult::Lose);
                            return;
                        }
                    }
                }
            }
        }


        for i in 0..self.enemies.len() {
            self.enemies[i].timer -= delta_time;
            if self.enemies[i].timer <= 0. {
                self.enemies[i].timer = 1.;
                let path = find_path(&self.map, (self.enemies[i].x, self.enemies[i].y), (self.px, self.py));
                if path.len() > 0 && !enemies_pos.contains(&(path[0])) {
                    self.enemies[i].x = path[0].0;
                    self.enemies[i].y = path[0].1;
                    if (self.enemies[i].x, self.enemies[i].y) == (self.px, self.py) {
                        self.state = GameState::GameOver(GameResult::Win);
                        return;
                    }
                }
            }
        }


    }

    pub fn draw(&self) {
        let size =  screen_width().min(screen_height());
        let offset_x = (screen_width() - size) / 2. + 10.;
        let offset_y = (screen_height() - size) / 2. + 10.;
        let t_size = (screen_height() - offset_y * 2.) / MAP_SIZE as f32;

        for i in 0..MAP_SIZE {
            for j in 0..MAP_SIZE {
                if self.map[i][j] == Tile::Wall {
                    draw_wall(offset_x + (j as f32 * t_size), offset_y + (i as f32 * t_size), t_size);
                } else if self.map[i][j] == Tile::Exit {
                    draw_exit(offset_x + (j as f32 * t_size), offset_y + (i as f32 * t_size), t_size/2.);
                }
            }
        }

        for enemy in &self.enemies {
            draw_ava(&self.enemy_ava, offset_x + (enemy.x as f32 * t_size), offset_y + (enemy.y as f32 * t_size), t_size );

        }
        draw_ava(&self.player_ava, offset_x + (self.px as f32 * t_size), offset_y + (self.py as f32 * t_size), t_size );

    }
}