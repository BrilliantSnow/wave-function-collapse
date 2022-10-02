mod tile;

use nannou::{prelude::*};
use tile::{four_scale, Tile};

struct Model {
    tileset: wgpu::Texture,
    grid: [usize; 25],
    tiles: [Tile; 4],
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().expect("Assets directory not found");
    let img_path = assets.join("images").join("colors").join("bluescale.png");
    let tileset = wgpu::Texture::from_path(app, img_path).expect("error reading assets/images/colors/bluescale.png");

    //testing displaying grid
    let grid = [
        1, 2, 2, 1, 1,
        2, 1, 2, 1, 0,
        3, 2, 2, 2, 1,
        2, 1, 2, 2, 1,
        1, 0, 1, 2, 1,
    ];

    let tiles = four_scale::new();
    
    Model {
        tileset,
        grid,
        tiles,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();

    const SIZE: f32 = 32.0;
    const SCALE: f32 = 5.0;
    const MARGIN: f32 = 0.0;

    let basis = -5.0 * SIZE * SCALE / 2.0;

    for y in 0..5 {
        for x in 0..5 {
            draw.texture(&model.tileset)
            .w_h(SIZE * SCALE, SIZE * SCALE)
            .area(model.tiles[model.grid[y * 5 + x]].texture)
            .x_y(basis + x as f32 * (SIZE * SCALE + MARGIN), basis + y as f32 * (SIZE * SCALE + MARGIN));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
