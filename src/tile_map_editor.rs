use std::ops::DerefMut;

use crate::tile_set::*;
use crate::utils::*;
use macroquad::prelude::*;
use macroquad::ui::*;
use macroquad::ui::widgets::*;

fn intager_editor(value: &mut usize, label: &str)
{
    const spacer: f32 = 3.0;
    const plus: &str = "+";
    const minus: &str = "-";

    let text = format!("{}: {}", label, value);
    let text_size = root_ui().calc_size(&text);
    let plus_size = root_ui().calc_size(plus);

    root_ui().label(None, &text);
    root_ui().same_line(text_size.x + spacer);
    if root_ui().button(None, plus) && *value < usize::max_value()
    {
        *value += 1;
    }

    root_ui().same_line(text_size.x + spacer + plus_size.x + spacer);
    if root_ui().button(None, minus) && *value > usize::min_value()
    {
        *value -= 1;
    }
}

pub struct TileMapEditor<'map>
{
    current_tile: Option<usize>,
    tile_options: Vec<String>,
    entity: &'map mut TileMapEntity,
    pub camera: Camera2D
}

impl<'map> TileMapEditor<'map>
{
    pub fn new(entity: &'map mut TileMapEntity, camera: Camera2D) -> Self
    {
        let mut tile_options: Vec<_> = entity.tile_map()
            .tiles()
            .iter()
            .map(|t| t.name.clone())
            .collect();

        tile_options.push(String::from("None"));

        TileMapEditor { current_tile: None, tile_options, entity, camera }
    }

    fn update_selected_tile(&mut self)
    {
        let options: Vec<_> = self.tile_options.iter().map(|t| t.as_str()).collect();
        let selected_tile_option = root_ui().combo_box(0, "Tiles:", &options, None);
        if selected_tile_option >= self.tile_options.len() - 1
        {
            self.current_tile = None
        }
        else
        {
            self.current_tile = Some(selected_tile_option);
        }
    }

    fn update_modify_map(&mut self)
    {
        if is_mouse_button_down(MouseButton::Left)
        {
            let mouse_world_pos = self.camera.screen_to_world(mouse_position().into());
            self.entity.set_from_pos(mouse_world_pos, self.current_tile);
        }
    }

    fn update_map_size(&mut self)
    {
        let initial = (self.entity.tile_map().width(), self.entity.tile_map().height());
        let mut size = initial;

        intager_editor(&mut size.0, "Vertical");
        intager_editor(&mut size.1, "Horizontal");

        if initial != size
        {
            let mut new_entity = TileMapEntity::new(pos, width, height, tile_size, tile_set, tiles, generator)
        }
    }

    pub fn update(&mut self)
    {
        self.update_selected_tile();
        self.update_map_size();
        self.update_modify_map();

        self.entity.render();
        self.entity.render_debug_lines();
    }

    
}