use core::fmt;

use itertools::Itertools;
use macroquad::prelude::*;
use macroquad::models::{Vertex, Mesh};

use crate::utils::Array2D;

const SUB_MAP_MAX_SIZE: UVec2 = UVec2{x: 10, y: 10};

#[derive(Debug, Clone)]
struct SubMap
{
    width: u16,
    height: u16,
    x: usize,
    y: usize,
    tiles: Vec<Tile>
}

impl SubMap
{
    fn new<F: Fn(usize, usize)->TileData>(width: u16, height: u16, x: usize, y: usize, gen: &F) -> Self
    {
        let mut tiles = vec![Tile{x: 0, y: 0, index: 0, data: TileData::default()}; (width * height) as usize];
        for xi in 0..width
        {
            for yi in 0..height
            {
                let data = gen(xi as usize + x, yi as usize + y);
                let index = yi * width + xi;
                tiles[index as usize] = Tile::new(xi as u16, yi as u16, index as u16, data);
            }
        }

        SubMap { width, height, x, y, tiles }
    }

    fn at(&self, x: u16, y: u16) -> &Tile
    {
        return &self.tiles[(y * self.width + x) as usize]
    }

    fn at_mut(&mut self, x: u16, y: u16) -> &mut Tile
    {
        return &mut self.tiles[(y * self.width + x) as usize]
    }

    fn to_mesh(&self, offset: Vec3, scale: f32, tileset: &TileSet) -> Mesh
    {
        let mut verticies = Vec::with_capacity(self.height as usize * self.width as usize * 4);
        let mut triangles = Vec::with_capacity(self.height as usize * self.width as usize * 6);
        let vertex_offset = offset + Vec3{x: self.x as f32, y: self.y as f32, z: 0.0} * scale;
        let mut triangle_offset = 0;
        for x in 0..self.width
        {
            for y in 0..self.height
            {
                if let Some(base_verts) = self.at(x, y).get_base_verticies(vertex_offset, scale, tileset)
                {
                    verticies.extend_from_slice(&base_verts);
                    triangles.extend_from_slice(&self.at(x, y).get_triangles(triangle_offset));
                    triangle_offset += 4;
                }
                if let Some(top_verts) = self.at(x, y).get_top_verticies(vertex_offset, scale, tileset)
                {
                    verticies.extend_from_slice(&top_verts);
                    triangles.extend_from_slice(&self.at(x, y).get_triangles(triangle_offset));
                    triangle_offset += 4;
                }
            }
        }

        Mesh { vertices: verticies, indices: triangles, texture: Some(tileset.texture) }
    }
}

#[derive(Debug, Clone)]
pub struct TileSet
{
    texture: Texture2D,
    width: u16,
    height: u16
}

impl TileSet
{
    pub fn new(texture: Texture2D, width: u16, height: u16) -> Self
    {
        TileSet { texture, width, height}
    }

    pub fn width(&self) -> u16 {self.width}
    pub fn height(&self) -> u16 {self.height}
    pub fn texture(&self) -> Texture2D {self.texture}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct TileData
{
    pub top: Option<UVec2>,
    pub base: Option<UVec2>
}

impl fmt::Display for TileData
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}; {}]", 
            match self.top 
            {
                Some(v) => format!("({}, {})", v.x, v.y),
                None => String::from("()")
            },
            match self.base 
            {
                Some(v) => format!("({}, {})", v.x, v.y),
                None => String::from("()")
            })
    }
}

impl TileData
{
    pub fn new(top: Option<UVec2>, base: Option<UVec2>) -> TileData
    {
        TileData { top, base }
    }
}

#[derive(Debug, Clone)]
pub struct Tile
{
    x: u16,
    y: u16,
    index: u16,
    pub data: TileData
}

impl Tile
{
    fn new(x: u16, y: u16, index: u16, data: TileData) -> Self
    {
        Tile { x, y, index, data }
    }

    pub fn x(&self) -> u16 {self.x}
    pub fn y(&self) -> u16 {self.y}
    pub fn index(&self) -> u16 {self.index}

    fn get_triangles(&self, offset: u16) -> [u16; 6]
    {
        let base_index = offset;
        [
            base_index,
            base_index + 1,
            base_index + 2,

            base_index + 1,
            base_index + 3,
            base_index + 2
        ]
    }

    fn get_top_verticies(&self, offset: Vec3, scale: f32, tileset: &TileSet) -> Option<[Vertex; 4]>
    {
        if let Some(id) = self.data.top
        {
            let gen = |xoffset, yoffset| 
            {
                let tile_size = Vec2{x: tileset.texture.width() / tileset.width as f32, y: tileset.texture.height() / tileset.height as f32};
                let uv_pos = Vec2{x: tile_size.x * (id.x as f32 + xoffset), y: tile_size.y * (id.y as f32 + yoffset)};
                let uv = Vec2{x: uv_pos.x / tileset.texture.width(), y: uv_pos.y / tileset.texture.height()};

                let pos = Vec3{x: self.x as f32 + xoffset, y: self.y as f32 + yoffset, z: 0.0} * scale + offset;
                Vertex{position: pos, uv, color: WHITE}
            };

            return Some([gen(0.0, 0.0), gen(1.0, 0.0), gen(0.0, 1.0), gen(1.0, 1.0)])
        }

        None
    }

    fn get_base_verticies(&self, offset: Vec3, scale: f32, tileset: &TileSet) -> Option<[Vertex; 4]>
    {
        if let Some(id) = self.data.base
        {
            let gen = |xoffset, yoffset|
            {
                let tile_size = Vec2{x: tileset.texture.width() / tileset.width as f32, y: tileset.texture.height() / tileset.height as f32};
                let uv_pos = Vec2{x: tile_size.x * (id.x as f32 + xoffset), y: tile_size.y * (id.y as f32 + yoffset)};
                let uv = Vec2{x: uv_pos.x / tileset.texture.width(), y: uv_pos.y / tileset.texture.height()};

                let pos = Vec3{x: self.x as f32 + xoffset, y: self.y as f32 + yoffset, z: 0.0} * scale + offset;
                Vertex{position: pos, uv, color: WHITE}
            };

            return Some([gen(0.0, 0.0), gen(1.0, 0.0), gen(0.0, 1.0), gen(1.0, 1.0)])
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct TileMap
{
    width: usize,
    height: usize,
    tiles: Vec<Vec<SubMap>>,
    tile_set: TileSet,
    sub_maps_x: usize,
    sub_maps_y: usize,
}

impl TileMap
{
    pub fn new<F: Fn(usize, usize)->TileData>(width: usize, height: usize, tile_set: TileSet, generator: &F) -> Self
    {
        let sub_maps_x = width / SUB_MAP_MAX_SIZE.x as usize + (if width % SUB_MAP_MAX_SIZE.x as usize != 0 {1} else {0});
        let sub_maps_y = height / SUB_MAP_MAX_SIZE.y as usize + (if height % SUB_MAP_MAX_SIZE.y as usize != 0 {1} else {0});

        let mut tiles = Vec::with_capacity(height);
        
        for y in 0..sub_maps_y
        {
            let mut row = Vec::with_capacity(width);
            for x in 0..sub_maps_x
            {
                let sub_width = 
                if x == sub_maps_x - 1 
                {
                    if width % SUB_MAP_MAX_SIZE.x as usize == 0
                    {
                        SUB_MAP_MAX_SIZE.x as u16
                    }
                    else
                    {
                        (width % SUB_MAP_MAX_SIZE.x as usize) as u16
                    }
                }
                else 
                {
                    SUB_MAP_MAX_SIZE.x as u16
                };
                
                let sub_height = 
                if y == sub_maps_y - 1 
                {
                    if height % SUB_MAP_MAX_SIZE.y as usize == 0
                    {
                        SUB_MAP_MAX_SIZE.y as u16
                    }
                    else
                    {
                        (height % SUB_MAP_MAX_SIZE.y as usize) as u16
                    }
                } 
                else 
                {
                    SUB_MAP_MAX_SIZE.y as u16
                };
                
                row.push(SubMap::new(sub_width, sub_height, x * SUB_MAP_MAX_SIZE.x as usize, y * SUB_MAP_MAX_SIZE.y as usize, generator));
            }
            tiles.push(row);
        }

        TileMap { width, height, tiles, tile_set, sub_maps_x, sub_maps_y}
        
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn tile_set(&self) -> &TileSet {&self.tile_set}

    fn at(&self, x: usize, y: usize) -> &Tile
    {
        let map_x = x / SUB_MAP_MAX_SIZE.x as usize;
        let map_y = y / SUB_MAP_MAX_SIZE.y as usize;

        let tiles = &self.tiles[map_y][map_x];

        let map_index_x = (x % tiles.width as usize) as u16;
        let map_index_y = (y % tiles.height as usize) as u16;

        return &(self.tiles[map_y][map_x].at(map_index_x, map_index_y))
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Tile
    {
        let mut map_x = x / SUB_MAP_MAX_SIZE.x as usize;
        let mut map_y = y / SUB_MAP_MAX_SIZE.y as usize;
        
        if map_x >= self.sub_maps_x {map_x = self.sub_maps_x - 1}
        if map_y >= self.sub_maps_y {map_y = self.sub_maps_y - 1}

        let tiles = &self.tiles[map_y][map_x];

        let map_index_x = (x % tiles.width as usize) as u16;
        let map_index_y = (y % tiles.height as usize) as u16;

        return self.tiles[map_y][map_x].at_mut(map_index_x, map_index_y)
    }
    
    pub fn to_mesh(&self, offset: Vec3, scale: f32) -> Vec<Mesh>
    {
        self.tiles.iter()
                  .flatten()
                  .map(|m| m.to_mesh(offset, scale, &self.tile_set))
                  .collect()
    }
}

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
    pub fn new<F: Fn(usize, usize)->TileData>(pos: Vec3, width: usize, height: usize, tile_size: f32, tile_set: TileSet, generator: &F) -> Self
    {
        let map = TileMap::new(width, height, tile_set, generator);
        let meshes = map.to_mesh(pos, tile_size);
        Self { pos, tile_size, map, meshes }
    }

    pub fn from_array2d(base: &Array2D<TileData>, tile_set: TileSet, pos: Vec3, tile_size: f32) -> Self
    {
        Self::new(pos, base.width(), base.height(), tile_size, tile_set, &|x, y| base.at(x, y).clone())
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
        (self.map.width, self.map.height)
    }

    pub fn set_without_update(&mut self, x: usize, y: usize, data: TileData)
    {
        self.map.at_mut(x, y).data = data;
    }

    pub fn set(&mut self, x: usize, y: usize, data: TileData)
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

    pub fn set_from_pos_without_update(&mut self, pos: Vec2, data: TileData)
    {
        let size = self.size();
        if pos.x < self.pos.x || pos.x > self.pos.x + size.x ||
           pos.y < self.pos.y || pos.y > self.pos.y + size.y
        {
            panic!("Tried to index an out of range position {}", pos);
        }

        let relative = pos - self.pos.truncate();
        let grid_pos = (relative / self.tile_size).floor().as_uvec2();
        self.map.at_mut(grid_pos.x as usize, grid_pos.y as usize).data = data;
    }

    pub fn update(&mut self)
    {
        self.meshes = self.map.to_mesh(self.pos, self.tile_size);
    }

    pub fn set_from_pos(&mut self, pos: Vec2, data: TileData)
    {
        self.set_from_pos_without_update(pos, data);
        self.update();
    }

    pub fn render(&self)
    {
        for mesh in &self.meshes
        {
            draw_mesh(mesh)
        }
    }
}