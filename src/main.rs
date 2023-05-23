mod tile_set;
mod utils;
mod wfc;
mod wfc_renderer;
mod file_system;
mod tile_map;
mod gui;
mod tile_map_entity;

use macroquad::{prelude::*, ui::root_ui};
use gui::{tile_map_editor::TileMapEditor, pattern_viewer::{PatternViewer}, intager_editor::*};
use tile_set::*;
use utils::*;
use wfc_renderer::*;
use tile_map::*;
use tile_map_entity::*;

const MAP_TO_SCREEN_SCALE: f32 = 0.7;

async fn get_wfc_entity(tiles: &Vec<TileData>, model: &Array2D<usize>, error_tile: Option<usize>, width: usize, height: usize) -> WFCEntity
{
    let tileset = get_tile_set().await;
    let tile_size = get_map_tile_size(width, height, MAP_TO_SCREEN_SCALE);
    let map_pos = get_map_pos(width, height, tile_size, false);

    WFCEntity::new(tiles.clone(), &model, 2, tileset, map_pos, tile_size, width, height, 42, error_tile)
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

fn get_map_pos(width: usize, height: usize, tile_size: f32, add_x_offset: bool) -> Vec3
{
    let screen_center = Vec2{x: screen_width() / 2.0, y: screen_height() / 2.0};
    Vec3
    {
        x: screen_center.x - (width as f32 * tile_size / 2.0) + if add_x_offset {screen_width() / 8.0} else {0.0},
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

async fn get_model_entity(tiles: &Vec<TileData>, model: &Array2D<Option<usize>>, add_x_offset: bool) -> TileMapEntity
{
    let tile_size = get_map_tile_size(model.width(), model.height(), MAP_TO_SCREEN_SCALE);
    let pos = get_map_pos(model.width(), model.height(), tile_size, add_x_offset);
    let tileset = get_tile_set().await;

    TileMapEntity::from_array2d(tiles.clone(), &model, tileset, pos, tile_size)
}

fn get_tiles() -> Vec<TileData>
{
    let grass_tile =    TileData::new(vec![tile_index(3, 0)], "Grass", "G");
    let error_tile =    TileData::new(vec![tile_index(0, 43)], "Unknown", "");
    let house_tile =    TileData::new(vec![tile_index(3, 0), tile_index(0, 1)], "House", "H");
    let trees =         TileData::new(vec![tile_index(2, 0), tile_index(4, 0)], "Trees", "T");
    let mountains =     TileData::new(vec![tile_index(0, 0), tile_index(5, 1)], "Mountains", "M");
    let bushes =        TileData::new(vec![tile_index(3, 0), tile_index(6, 3)], "Bushes", "B");
    
    let bridge = TileData::new(vec![tile_index(0, 0), tile_index(2, 14), tile_index(4, 34)], "Bridge", "Br");

    vec![grass_tile, house_tile, trees, mountains, bushes, bridge, error_tile]
}

fn get_editor<'a>(entity: &'a mut TileMapEntity, camera: Camera2D) -> TileMapEditor<'a, fn(&mut TileMapEntity)>
{
    TileMapEditor::new(entity, camera, Some::<fn(&mut TileMapEntity)>(|map| {
        let tile_size = get_map_tile_size(map.tile_map().width(), map.tile_map().height(), MAP_TO_SCREEN_SCALE);
        let map_pos = get_map_pos(map.tile_map().width(), map.tile_map().height(), tile_size, true);
        map.pos = map_pos;
        map.tile_size = tile_size;
    }))
}

async fn run_editor(entity: &mut TileMapEntity, camera: Camera2D)
{
    entity.pos = get_map_pos(entity.tile_map().width(), entity.tile_map().height(), entity.tile_size, true);
    entity.update();

    let mut editor = get_editor(entity, camera);
    loop {
        clear_background(BLUE);
        let close_editor = editor.update();
        next_frame().await;
        if close_editor
        {
            entity.pos = get_map_pos(entity.tile_map().width(), entity.tile_map().height(), entity.tile_size, false);
            entity.update();
            break;
        }
    }
}

async fn run_pattern_viewer(entity: &WFCEntity)
{
    let mut pattern_viewer = PatternViewer::new(entity.tiles(), entity.patterns(), entity.tile_set());
    loop 
    {
        clear_background(BLUE);
        let wants_to_close = !pattern_viewer.update();
        next_frame().await;
        if wants_to_close {break;}
    }
}

async fn try_run_wfc(tiles: &Vec<TileData>, model: &Array2D<Option<usize>>, error_tile: Option<usize>) -> bool
{
    let checked_model = 
        if model.into_iter().any(|t| t.1.is_none()) 
        {
            None
        }
        else 
        {
            Some(Array2D::new(model.width(), model.height(), &|x, y| model.at(x, y).unwrap()))
        };
    
    let Some(wfc_model) = checked_model else { return false; };

    let mut output_width = 10;
    let mut output_height = 10;
    let mut wfc_entity = get_wfc_entity(tiles, &wfc_model, error_tile, output_width, output_height).await;

    loop 
    {
        clear_background(BLUE);
        let close = root_ui().button(None, "Close");

        let mut was_size_changed = false;
        was_size_changed |= intager_editor(&mut output_width, "Width:", &mut root_ui());
        was_size_changed |= intager_editor(&mut output_height, "Height:", &mut root_ui());

        if was_size_changed
        {
            wfc_entity = get_wfc_entity(tiles, &wfc_model, error_tile, output_width, output_height).await
        }

        if root_ui().button(None, "Display Patterns")
        {
            run_pattern_viewer(&wfc_entity).await;
        }

        if root_ui().button(None, "Run WFC")
        {
            wfc_entity.collapse_full();
        }

        wfc_entity.render();

        next_frame().await;
        if close
        {
            break true;
        }
    }
}

async fn main_loop(tiles: Vec<TileData>, mut entity: TileMapEntity, camera: Camera2D, error_tile: Option<usize>)
{
    let mut show_error_message = false;
    loop {
        if root_ui().button(None, "Load Map")
        {
            run_editor(&mut entity, camera).await;
        }

        if root_ui().button(None, "Run WFC")
        {
            let model = Array2D::new(entity.tile_map().width(), entity.tile_map().height(), &|x, y| 
            {
                entity.tile_map().at(x, y).data
            });
            let was_run = try_run_wfc(&tiles, &model, error_tile).await;
            show_error_message = !was_run;
        }

        if show_error_message
        {
            root_ui().same_line(0.0);
            root_ui().label(None, "All tiles must be filled in the model");
        }

        clear_background(BLUE);
        entity.render_with_debug_lines();
        next_frame().await
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let camera = &mut Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: screen_width(), h: screen_height() });
    set_camera(camera);
    
    let tiles = get_tiles();
    
    let model = Array2D::<Option<usize>>::new_default(10, 10);
    let entity = get_model_entity(&tiles, &model, false).await;

    let error_tile_index = tiles.len() - 1;
    main_loop(tiles, entity, *camera, Some(error_tile_index)).await;
}
