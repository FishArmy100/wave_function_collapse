use macroquad::prelude::*;
use crate::utils::*;
use crate::tile_set::*;
use crate::tile_map::*;

pub struct TileMapEntity
{
    pub pos: Vec3,
    pub tile_size: f32,
    map: TileMap,
    meshes: Vec<Mesh>
}

impl Clone for TileMapEntity
{
    fn clone(&self) -> Self {
        Self 
        { 
            pos: self.pos.clone(), 
            tile_size: self.tile_size.clone(), 
            map: self.map.clone(), 
            meshes: (&self.meshes)
                        .into_iter()
                        .map(|m| Mesh 
                            {
                                vertices: m.vertices.clone(), 
                                indices: m.indices.clone(), 
                                texture: m.texture.clone()
                            })
                        .collect_vec()
        }
    }
}

impl fmt::Debug for TileMapEntity
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TileMapEntity")
            .field("pos", &self.pos)
            .field("tile_size", &self.tile_size)
            .field("map", &self.map)
            .finish_non_exhaustive()
    }
}

impl TileMapEntity
{
    pub fn new<F: Fn(usize, usize)->Option<usize>>(pos: Vec3, width: usize, height: usize, tile_size: f32, tile_set: TileSet, tiles: Vec<TileData>, generator: &F) -> Self
    {
        let map = TileMap::new(width, height, tile_set, tiles, generator);
        let meshes = map.to_mesh(pos, tile_size);
        Self { pos, tile_size, map, meshes }
    }

    pub fn from_tile_map(map: TileMap, pos: Vec3, tile_size: f32) -> TileMapEntity
    {
        let meshes = map.to_mesh(pos, tile_size);
        Self { pos, tile_size, map, meshes }
    }

    pub fn from_array2d(tiles: Vec<TileData>, base: &Array2D<Option<usize>>, tile_set: TileSet, pos: Vec3, tile_size: f32) -> Self
    {
        Self::new(pos, base.width(), base.height(), tile_size, tile_set, tiles, &|x, y| *base.at(x, y))
    }

    pub fn tile_map(&self) -> &TileMap
    {
        &self.map
    }

    pub fn tile_size(&self) -> f32
    {
        self.tile_size
    }

    pub fn size(&self) -> Vec2
    {
        vec2(self.map.width() as f32 * self.tile_size, self.map.height() as f32 * self.tile_size)
    }

    pub fn tile_count(&self) -> (usize, usize)
    {
        (self.map.width(), self.map.height())
    }

    pub fn set_without_update(&mut self, x: usize, y: usize, data: Option<usize>)
    {
        self.map.at_mut(x, y).data = data;
    }

    pub fn set(&mut self, x: usize, y: usize, data: Option<usize>)
    {
        self.set_without_update(x, y, data);
        self.update();
    }

    pub fn get_from_pos(&self, pos: Vec2) -> Option<&Tile>
    {
        let size = self.size();
        if pos.x < self.pos.x || pos.x > self.pos.x + size.x ||
           pos.y < self.pos.y || pos.y > self.pos.y + size.y
        {
            return None;
        }

        let relative = pos - self.pos.truncate();
        let grid_pos = (relative / self.tile_size).floor().as_uvec2();
        Some(self.map.at(grid_pos.x as usize, grid_pos.y as usize))
    }

    pub fn set_from_pos_without_update(&mut self, pos: Vec2, data: Option<usize>) -> bool
    {
        let size = self.size();
        if pos.x < self.pos.x || pos.x > self.pos.x + size.x ||
           pos.y < self.pos.y || pos.y > self.pos.y + size.y
        {
            return false;
        }

        let relative = pos - self.pos.truncate();
        let grid_pos = (relative / self.tile_size).floor().as_uvec2();
        self.map.at_mut(grid_pos.x as usize, grid_pos.y as usize).data = data;
        true
    }

    pub fn update(&mut self)
    {
        self.meshes = self.map.to_mesh(self.pos, self.tile_size);
    }

    pub fn set_from_pos(&mut self, pos: Vec2, data: Option<usize>) -> bool
    {
        if self.set_from_pos_without_update(pos, data)
        {
            self.update();
            true
        }
        else
        {
            false
        }
        
    }

    pub fn render(&self)
    {
        for mesh in &self.meshes
        {
            draw_mesh(mesh)
        }
    }

    pub fn render_debug_lines(&self)
    {
        let line_color = BLACK;
        let line_thickness = 3.0;

        let pos = self.pos;
        let size = vec2(self.map.width() as f32 * self.tile_size(), self.map.height() as f32 * self.tile_size());
        
        // draw outline
        draw_rectangle_lines(pos.x, pos.y, size.x, size.y, line_thickness * 2.0, line_color);

        // vertical lines
        for x in 1..self.map.width()
        {
            let x_pos = pos.x + self.tile_size() * x as f32;
            draw_line(x_pos, pos.y, x_pos, pos.y + size.y, line_thickness, line_color);
        }

        // horizontal lines
        for y in 1..self.map.height()
        {
            let y_pos = pos.y + self.tile_size() * y as f32;
            draw_line(pos.x, y_pos, pos.x + size.x, y_pos, line_thickness, line_color);
        }
    }
}