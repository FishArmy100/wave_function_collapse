mod tile_set;
use macroquad::{prelude::*, rand::rand};
use tile_set::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let house_gen = || {if rand() % 2 == 0 {None} else {Some(uvec2(0, 1))}};
    let tiles = TileMap::new(32, 25, &|x, y| (Some(uvec2(3, 0)), house_gen()));
    
    let offset = Vec3{x: 100.0, y: 100.0, z: 0.0 };
    
    let texture = load_texture("C:\\dev\\Rust\\wave_function_collapse\\resources\\medieval_pixel_art_tileset\\TileSet.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let tileset = TileSet::new(texture, 7, 44 + 8);

    let meshes = tiles.to_mesh(offset, 20.0, &tileset);

    loop {
        clear_background(RED);
        for mesh in &meshes
        {
            draw_mesh(&mesh);
        }

        next_frame().await
    }
}
