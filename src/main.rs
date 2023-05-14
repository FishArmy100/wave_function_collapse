mod tile_set;
mod utils;
mod wfc;
mod wfc_renderer;
mod file_system;
mod tile_map_editor;
mod tile_map;

use macroquad::prelude::*;
use tile_map_editor::TileMapEditor;
use tile_set::*;
use utils::*;
use wfc_renderer::*;
use tile_map::*;

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
    let path = "C:\\dev\\Rust\\wave_function_collapse\\resources\\medieval_pixel_art_tileset\\TileSet.png";
    let tileset = TileSet::from_file(path, 7, 44 + 8).await;

    let temp_path = "temp.txt";

    file_system::serialize_to_file(&tileset, temp_path);
    let tileset = file_system::deserialize_from_file::<TileSet>(temp_path);
    
    tileset
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

fn get_map_tile_size(width: usize, height: usize, map_to_screen_scale: f32) -> f32
{
    let smallest = width.max(height);
    let smallest_screen_len = screen_width().max(screen_height());
    smallest_screen_len * map_to_screen_scale / smallest as f32
}

async fn get_model_entity(tiles: &Vec<TileData>, model: &Array2D<Option<usize>>) -> TileMapEntity
{
    let tile_size = get_map_tile_size(model.width(), model.height(), 0.7);
    let pos = get_map_pos(model.width(), model.height(), tile_size);
    let tileset = get_tile_set().await;

    TileMapEntity::from_array2d(tiles.clone(), &model, tileset, pos, tile_size)
}

fn get_tiles() -> Vec<TileData>
{
    let grass_tile =    TileData::new(None, Some(tile_index(3, 0)), "Grass", "G");
    let error_tile =    TileData::new(None, Some(tile_index(0, 43)), "Unknown", "");
    let house_tile =    TileData::new(Some(tile_index(0, 1)), Some(tile_index(3, 0)), "House", "H");
    let trees =         TileData::new(Some(tile_index(4, 0)), Some(tile_index(2, 0)), "Trees", "T");
    let mountains =     TileData::new(Some(tile_index(5, 1)), Some(tile_index(0, 0)), "Mountains", "M");
    let bushes =        TileData::new(Some(tile_index(6, 3)), Some(tile_index(3, 0)), "Bushes", "B");
    
    
    vec![grass_tile, house_tile, trees, mountains, bushes, error_tile]
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let camera = &mut Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: screen_width(), h: screen_height() });
    set_camera(camera);
    
    let tiles = get_tiles();
    
    let model = Array2D::<Option<usize>>::new_default(13, 13);
    let mut entity = get_model_entity(&tiles, &model).await;

    let mut editor = TileMapEditor::new(&mut entity, *camera, Some::<fn(&mut TileMapEntity)>(|map| {
        let tile_size = get_map_tile_size(map.tile_map().width(), map.tile_map().height(), 0.7);
        let map_pos = get_map_pos(map.tile_map().width(), map.tile_map().height(), tile_size);
        map.pos = map_pos;
        map.tile_size = tile_size;
    }));

    loop {
        clear_background(BLUE);
        editor.update();
        next_frame().await
    }
}
