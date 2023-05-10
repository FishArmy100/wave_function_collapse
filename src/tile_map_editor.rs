use crate::tile_map::*;
use macroquad::prelude::*;
use macroquad::ui::*;

fn intager_editor(value: &mut usize, label: &str)
{
    const SPACER: f32 = 3.0;
    const PLUS: &str = "+";
    const MINUS: &str = "-";

    let text = format!("{}: {}", label, value);
    let text_size = root_ui().calc_size(&text);
    let plus_size = root_ui().calc_size(PLUS);

    root_ui().label(None, &text);
    root_ui().same_line(text_size.x + SPACER);
    if root_ui().button(None, PLUS) && *value < usize::max_value()
    {
        *value += 1;
    }

    root_ui().same_line(text_size.x + SPACER + plus_size.x + SPACER);
    if root_ui().button(None, MINUS) && *value > usize::min_value()
    {
        *value -= 1;
    }
}

pub struct TileMapEditor<'map, TFunc> where TFunc : for<'a> Fn(&'a mut TileMapEntity)
{
    current_tile: Option<usize>,
    tile_options: Vec<String>,
    entity: &'map mut TileMapEntity,
    pub camera: Camera2D,
    pub on_map_size_changed: Option<TFunc>
}

impl<'map, TFunc> TileMapEditor<'map, TFunc> where TFunc : for<'a> Fn(&'a mut TileMapEntity)
{
    pub fn new(entity: &'map mut TileMapEntity, camera: Camera2D, on_map_size_changed: Option<TFunc>) -> Self
    {
        let mut tile_options: Vec<_> = entity.tile_map()
            .tiles()
            .iter()
            .map(|t| t.name.clone())
            .collect();

        tile_options.push(String::from("None"));

        TileMapEditor { current_tile: None, tile_options, entity, camera, on_map_size_changed }
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

    fn update_map_size(&mut self) -> bool
    {
        let initial = (self.entity.tile_map().width(), self.entity.tile_map().height());
        let mut size = initial;

        intager_editor(&mut size.1, "Vertical");
        intager_editor(&mut size.0, "Horizontal");

        if initial != size
        {
            let pos = self.entity.pos;
            let tile_size = self.entity.tile_size;
            let tile_set = self.entity.tile_map().tile_set().clone();
            let tiles = self.entity.tile_map().tiles().clone();
            let new_entity = TileMapEntity::new(pos, size.0, size.1, tile_size, tile_set, tiles, &|x, y| 
            {
                if x < initial.0 && y < initial.1
                {
                    self.entity.tile_map().at(x, y).data
                }
                else
                {
                    None
                }
            });

            *self.entity = new_entity;
            return true;
        }

        false
    }

    pub fn update(&mut self)
    {
        self.update_selected_tile();
        let size_changed = self.update_map_size();
        self.update_modify_map();

        if size_changed
        {
            if let Some(func) = &self.on_map_size_changed
            {
                func(self.entity);
            }
            self.entity.update();
        }

        self.entity.render();
        self.entity.render_debug_lines();
    }
}