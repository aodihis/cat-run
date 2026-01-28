use macroquad::prelude::{render_target, Camera2D, Rect};
use macroquad::window::{screen_height, screen_width};
use crate::constant::{MAP_SIZE, T_SIZE};
use crate::GameState;
use crate::helpers::draw_helpers::draw_wall;

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
    Exit,
}

pub type Coord = (i32, i32);
pub struct Game {
    pub state: GameState,
    map: [[Tile; MAP_SIZE]; MAP_SIZE],
}

impl Game {
    pub fn new() -> Self {
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
        Self {
            map,
            state: GameState::Menu,
        }
    }

    pub fn update(&mut self) {

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
                }
            }
        }
    }
}