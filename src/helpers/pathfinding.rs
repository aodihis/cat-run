use std::cmp::Reverse;
use std::collections::BinaryHeap;
use crate::constant::MAP_SIZE;
use crate::game::Tile;

pub fn find_path(map: &[[Tile; MAP_SIZE]; MAP_SIZE], start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {

    let mut vis = [[i32::MAX;MAP_SIZE];MAP_SIZE];
    vis[start.1][start.0] = 0;
    let mut prev = [[None;MAP_SIZE];MAP_SIZE];

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start.0, start.1)));

    while let Some(Reverse((cost, x, y))) = heap.pop() {
        if x == end.0 && y == end.1 {
            let mut path: Vec<(usize, usize)> = vec![];
            let mut current = end;
            while current != start {
                path.push(current);
                if let Some(c) = prev[current.1][current.0] {
                    current = c;
                } else {
                    return vec![];
                }
            }
            path.reverse();
            return path;
        }
        for (dx, dy) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < MAP_SIZE as i32 && ny >= 0 && ny < MAP_SIZE as i32
                && map[ny as usize][nx as usize] == Tile::Floor
                && vis[ny as usize][nx as usize] > cost + 1 {
                vis[ny as usize][nx as usize] = cost + 1;
                heap.push(Reverse((cost + 1, nx as usize, ny as usize)));
                prev[ny as usize][nx as usize] = Some((x,y));
            }
        }
    }
    vec![]
}