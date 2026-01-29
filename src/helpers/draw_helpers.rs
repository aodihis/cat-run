use macroquad::prelude::*;
use crate::constant::T_SIZE;

pub fn draw_wall(x: f32, y: f32, size: f32) {
    draw_rectangle(
        x,
        y ,
        size, size,
        DARKBROWN
    )
}

pub fn draw_exit(x: f32, y: f32, r: f32) {
    draw_circle(
        x + r,
        y + r,
        r,
        BLUE
    )
}

pub fn draw_ava(ava: &Texture2D, x: f32, y: f32, t_size: f32) {
    draw_texture_ex(ava, x, y,
                    WHITE, DrawTextureParams {
            dest_size: Some(vec2(t_size, t_size)),
            ..Default::default()
        }
    );
}


pub fn to_screen(x: usize, y: usize) -> (f32, f32) {
    (
        x as f32 * T_SIZE,
        y as f32 * T_SIZE,
    )
}