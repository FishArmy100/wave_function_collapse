mod tile_set;
use macroquad::prelude::*;
use tile_set::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let tiles = TileMap::new(34, 34, |x, y| (uvec2(6, 0), None));
    
    let offset = Vec3{x: 100.0, y: 100.0, z: 0.0 };
    
    let texture = load_texture("C:\\dev\\Rust\\wave_function_collapse\\resources\\medieval_pixel_art_tileset\\TileSet.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let tileset = TileSet::new(texture, 7, 44 + 8);

    let mesh = tiles.to_mesh(offset, 10.0, &tileset);

    loop {
        clear_background(RED);
        draw_mesh(&mesh);
        next_frame().await
    }
}
