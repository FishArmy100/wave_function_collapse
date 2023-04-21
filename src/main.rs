mod tile_set;
mod utils;
mod wfc;

use macroquad::{prelude::*, rand::rand};
use tile_set::*;
use utils::Array2D;
use wfc::{Pattern};

use crate::utils::ArrayPos;

async fn get_tilemap() -> TileMapEntity
{
    let texture = load_texture("C:\\dev\\Rust\\wave_function_collapse\\resources\\medieval_pixel_art_tileset\\TileSet.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let tileset = TileSet::new(texture, 7, 44 + 8);

    let house_gen = || {if rand() % 2 == 0 {Some(uvec2(0, 1))} else {None}};
    let mut map_entity = TileMapEntity::new(Vec3::ZERO, 50, 50, 10.0, tileset, &|x, y| (Some(uvec2(3, 0)), None));

    let screen_center = Vec2{x: screen_width() / 2.0, y: screen_height() / 2.0};
    let map_pos = Vec2
    {
        x: screen_center.x - (map_entity.size().x / 2.0),
        y: screen_center.y - (map_entity.size().y  / 2.0)
    };

    map_entity.pos = map_pos.extend(0.0);
    return map_entity
}

fn test_wfc()
{
    let mut array = Array2D::<u32>::new_default(5, 5);

    *array.at_mut(2, 2) = 1;
    let pattern = Pattern::from_grid(ArrayPos::new(2, 2), 2, &array);

    println!("{:#?}", pattern.unwrap().data().at(1, 1));
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut map = get_tilemap().await;
    let mut meshes = map.get_mesh();

    let camera = &mut Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: screen_width(), h: screen_height() });
    set_camera(camera);

    test_wfc();

    loop {
        if is_mouse_button_down(MouseButton::Left)
        {
            let local_pos = mouse_position();
            let pos = camera.screen_to_world(Vec2{x: local_pos.0, y: local_pos.1});
            //debug!("Mouse position: ({}, {})", pos.x, pos.y);
            if let Some(tile) = map.at_pos_mut(pos)
            {
                tile.top_id = Some(uvec2(0, 1));
                meshes = map.get_mesh(); // need to regenerate the mesh
            }
        }

        clear_background(BLUE);
        for mesh in &meshes
        {
            draw_mesh(&mesh);
        }

        next_frame().await
    }
}
