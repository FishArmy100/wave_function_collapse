mod tile_set;
mod utils;
mod wfc;
mod wfc_renderer;
mod file_system;
mod tile_map_editor;

use macroquad::prelude::*;
use macroquad::ui::*;
use tile_map_editor::TileMapEditor;
use tile_set::*;
use utils::*;
use wfc_renderer::*;

async fn get_wfc_entity(tiles: &Vec<TileData>, model: &Array2D<usize>, error_tile: Option<usize>) -> WFCEntity
{
    let tileset = get_tile_set().await;

    let tiles_x = 20;
    let tiles_y = 20;

    let tile_size = 20.0;

    let map_pos = get_map_pos(tiles_x, tiles_y, tile_size);

    WFCEntity::new(tiles.clone(), &model, 2, tileset, map_pos, tile_size, tiles_x, tiles_y, 42, error_tile)
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

async fn get_model_entity(tiles: &Vec<TileData>, model: &Array2D<Option<usize>>) -> TileMapEntity
{
    let tile_size = 50.0;
    let pos = get_map_pos(model.width(), model.height(), tile_size);
    let tileset = get_tile_set().await;

    TileMapEntity::from_array2d(tiles.clone(), &model, tileset, pos, tile_size)
}

fn get_tiles() -> Vec<TileData>
{
    let grass_tile = TileData::new(None, Some(uvec2(3, 0)), String::from("Grass"), String::from("G"));
    let house_tile = TileData::new(Some(uvec2(0, 1)), Some(uvec2(3, 0)), String::from("House"), String::from("H"));
    let error_tile = TileData::new(None, Some(uvec2(0, 43)), String::from("Unknown"), String::from(""));
    vec![grass_tile, house_tile, error_tile]
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let camera = &mut Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: screen_width(), h: screen_height() });
    set_camera(camera);
    
    let tiles = get_tiles();
    
    let model = Array2D::<Option<usize>>::new_default(5, 5);
    let mut entity = get_model_entity(&tiles, &model).await;
    let mut editor = TileMapEditor::new(&mut entity, *camera);

    loop {
        clear_background(BLUE);
        editor.update();
        next_frame().await
    }
}
