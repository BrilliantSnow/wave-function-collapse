use nannou::prelude::*;

type Neighbors = [Vec<usize>; 4];

pub struct Tile {
    pub neighbors: Neighbors,
    pub texture: Rect,
}

pub mod four_scale {
    use nannou::prelude::{Rect};

    use super::Tile;

    pub fn new() -> [Tile; 4] {
        [
            Tile {
                neighbors: [vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1]],
                texture: Rect::from_w_h(1.0, 0.25).shift_y(1.0 / 8.0),
            },
            Tile {
                neighbors: [vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]],
                texture: Rect::from_w_h(1.0, 0.25).shift_y(3.0 / 8.0),
            },
            Tile {
                neighbors: [vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
                texture: Rect::from_w_h(1.0, 0.25).shift_y(5.0 / 8.0),
            },
            Tile {
                neighbors: [vec![2, 3], vec![2, 3], vec![2, 3], vec![2, 3]],
                texture: Rect::from_w_h(1.0, 0.25).shift_y(7.0 / 8.0),
            },
        ]
    }
}
