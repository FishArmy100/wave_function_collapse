use crate::file_system;
use crate::tile_map::*;
use macroquad::prelude::*;
use macroquad::ui::*;
use macroquad::ui::widgets::Window;
use std::path::Path;
use std::fs;

const SAVE_PATH: &str = "maps";
const EXTENSION: &str = "json";

pub struct TileMapEditor<'map, TFunc> where TFunc : for<'a> Fn(&'a mut TileMapEntity)
{
    current_tile: Option<usize>,
    tile_options: Vec<String>,
    entity: &'map mut TileMapEntity,
    pub camera: Camera2D,
    pub on_map_size_changed: Option<TFunc>,

    is_saving: bool,
    is_loading: bool,
    map_name: String,
    loaded_maps: Option<Vec<String>>
}

impl<'map, TFunc> TileMapEditor<'map, TFunc> where TFunc : for<'a> Fn(&'a mut TileMapEntity)
{
    pub fn new(entity: &'map mut TileMapEntity, camera: Camera2D, on_map_size_changed: Option<TFunc>) -> Self
    {
        let tile_options: Vec<_> = entity.tile_map()
            .tiles()
            .iter()
            .map(|t| t.name.clone())
            .collect();

        TileMapEditor 
        { 
            current_tile: None, 
            tile_options, 
            entity, 
            camera, 
            on_map_size_changed,
            is_saving: false, 
            is_loading: false, 
            map_name: String::from(""), 
            loaded_maps: None 
        }
    }

    fn update_selected_tile(&mut self)
    {
        let options: Vec<_> = self.tile_options.iter().map(|t| t.as_str()).collect();
        let selected_tile_option = root_ui().combo_box(0, "Tiles:", &options, None);
        self.current_tile = Some(selected_tile_option);
    }

    fn update_modify_map(&mut self)
    {
        let mouse_pos = mouse_position().into();

        if !self.is_loading && !self.is_saving && !root_ui().is_mouse_over(mouse_pos)
        {
            if is_mouse_button_down(MouseButton::Left)
            {
                let mouse_world_pos = self.camera.screen_to_world(mouse_pos);
                self.entity.set_from_pos(mouse_world_pos, self.current_tile);
            }

            if is_mouse_button_down(MouseButton::Right)
            {
                let mouse_world_pos = self.camera.screen_to_world(mouse_pos);
                self.entity.set_from_pos(mouse_world_pos, None);
            }
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

    fn update_save_map_ui(&mut self)
    {
        if root_ui().button(None, "Save")
        {
            self.is_saving = true;
        }

        if self.is_saving
        {
            self.is_saving = Window::new(hash!(), vec2(0., 0.), vec2(screen_width() / 2., screen_height() / 2.))
                .label("Save Map")
                .close_button(true)
                .ui(&mut root_ui(), |ui|
                    {
                        ui.input_text(hash!("Map Name"), "Map Name:", &mut self.map_name);
                        if ui.button(None, "Save") && !self.map_name.is_empty()
                        {
                            fs::create_dir_all(Path::new(SAVE_PATH)).expect("Failed to create directory");
                            let path = SAVE_PATH.to_owned() + "\\" + &self.map_name + "." + EXTENSION;
                            file_system::serialize_to_file(&self.entity.tile_map(), path.as_str())
                        }
                    })
        }
    }

    fn update_load_map_ui(&mut self) -> bool
    {
        if root_ui().button(None, "Load")
        {
            self.loaded_maps = Some(get_saved_maps());
            self.is_loading = true;
        }

        if self.is_loading
        {
            let mut map_loaded = false;

            let closed = !Window::new(hash!(), vec2(0., 0.), vec2(screen_width() / 2., screen_height() / 2.))
                .label("Load Map")
                .close_button(true)
                .ui(&mut root_ui(), |ui|
                    {
                        if let Some(maps) = &self.loaded_maps 
                        {
                            for map in maps
                            {
                                ui.label(None, &map);
                                ui.same_line(0.0);
                                if ui.button(None, "Load")
                                {
                                    let map: TileMap = file_system::deserialize_from_file(&map);
                                    let entity = TileMapEntity::from_tile_map(map, self.entity.pos, self.entity.tile_size);
                                    *self.entity = entity;
                                    map_loaded = true;
                                }

                                self.entity.update();
                            }
                        }
                    });
            
            if map_loaded || closed {self.is_loading = false;}

            return map_loaded;
        }

        false
    }

    pub fn update(&mut self)
    {
        self.update_modify_map();
        self.update_selected_tile();
        let size_changed = self.update_map_size();
        self.update_save_map_ui();
        let map_loaded = self.update_load_map_ui();

        if size_changed || map_loaded
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

fn get_saved_maps() -> Vec<String>
{
    let dir_path = ".\\".to_owned() + SAVE_PATH;
    let files = fs::read_dir(&dir_path).unwrap();

    let mut map_files: Vec<String> = Vec::new();
    
    for file in files 
    {
        if file.as_ref().is_ok_and(|f| f.path().extension().is_some_and(|p| p == EXTENSION))
        {
            map_files.push(file.unwrap().path().as_path().to_str().unwrap().to_owned())
        }
    }
    
    map_files
}

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