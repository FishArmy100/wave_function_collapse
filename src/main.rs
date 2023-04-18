mod tile_set;
use macroquad::prelude::*;
use tile_set::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let tiles = TileMap::new(20, 20, |x, y| Tile{x, y, id: 0, top_id: None});
    
    let offset = Vec3{x: 100.0, y: 100.0, z: 0.0 };


    loop {
        clear_background(RED);
        let mesh = tiles.to_mesh(offset, 10.0);
        draw_mesh(&mesh);
        next_frame().await
    }
}
