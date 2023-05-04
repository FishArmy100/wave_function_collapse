mod tile_set;
mod utils;
mod wfc;
mod wfc_renderer;

use itertools::Itertools;
use macroquad::prelude::*;
use tile_set::*;
use utils::*;
use wfc_renderer::*;

async fn get_wfc_entity(model: &Array2D<TileData>) -> WFCEntity
{
    let tileset = get_tile_set().await;

    let tiles_x = 20;
    let tiles_y = 20;

    let tile_size = 20.0;

    let map_pos = get_map_pos(tiles_x, tiles_y, tile_size);

    let error_tile = TileData::new(None, Some(uvec2(0, 43)));

    WFCEntity::new(&model, 2, tileset, map_pos, tile_size, tiles_x, tiles_y, 42, error_tile)
}

async fn get_tile_set() -> TileSet
{
    let texture = load_texture("C:\\dev\\Rust\\wave_function_collapse\\resources\\medieval_pixel_art_tileset\\TileSet.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    TileSet::new(texture, 7, 44 + 8)
}

fn get_map_pos(width: usize, height: usize, tile_size: f32) -> Vec3
{
    let screen_center = Vec2{x: screen_width() / 2.0, y: screen_height() / 2.0};
    Vec3
    {
        x: screen_center.x - (width as f32 * tile_size / 2.0),
        y: screen_center.y - (height as f32 * tile_size  / 2.0),
        z: 0.0
    }
}

async fn get_model_entity(model: &Array2D<TileData>) -> TileMapEntity
{
    let tile_size = 50.0;
    let pos = get_map_pos(model.width(), model.height(), tile_size);
    let tileset = get_tile_set().await;

    TileMapEntity::from_array2d(&model, tileset, pos, tile_size)
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let camera = &mut Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: screen_width(), h: screen_height() });
    set_camera(camera);
    
    let grass_tile = TileData::new(None, Some(uvec2(3, 0)));
    let house_tile = TileData::new(Some(uvec2(0, 1)), Some(uvec2(3, 0)));
    let mut model = Array2D::<TileData>::new(5, 5, vec![grass_tile; 5 * 5]);
    *model.at_mut(2, 2) = house_tile;
    let mut entity = get_wfc_entity(&model).await;
    entity.collapse_full();

    loop {
        clear_background(BLUE);
        entity.render();
        next_frame().await
    }
}
