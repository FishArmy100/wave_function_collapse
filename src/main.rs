mod tile_set;
mod utils;
mod wfc;
mod wfc_renderer;
use macroquad::prelude::*;
use tile_set::*;
use utils::Array2D;
use wfc_renderer::*;

async fn get_wfc_entity() -> WFCEntity
{
    let texture = load_texture("C:\\dev\\Rust\\wave_function_collapse\\resources\\medieval_pixel_art_tileset\\TileSet.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let tileset = TileSet::new(texture, 7, 44 + 8);

    let tiles_x = 10;
    let tiles_y = 10;

    let tile_size = 30.0;

    let screen_center = Vec2{x: screen_width() / 2.0, y: screen_height() / 2.0};
    let map_pos = Vec2
    {
        x: screen_center.x - (tiles_x as f32 * tile_size / 2.0),
        y: screen_center.y - (tiles_y as f32 * tile_size  / 2.0)
    };

    let mut model = Array2D::<TileData>::new(5, 5, vec![TileData::new(Some(uvec2(3, 0)), None); 5 * 5]);
    *model.at_mut(2, 2) = TileData::new(Some(uvec2(3, 0)), Some(uvec2(0, 1)));

    WFCEntity::new(&model, 2, tileset, map_pos.extend(0.0), tile_size, tiles_x, tiles_y, 03928570298537)
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let camera = &mut Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: screen_width(), h: screen_height() });
    set_camera(camera);

    let mut entity = get_wfc_entity().await;

    loop {
        clear_background(BLUE);

        if is_key_pressed(KeyCode::Space)
        {
            entity.step(1);
        }

        entity.render();

        next_frame().await
    }
}
