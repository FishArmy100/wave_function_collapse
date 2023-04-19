use macroquad::prelude::*;
use macroquad::models::{Vertex, Mesh};

const SUB_MAP_MAX_SIZE: UVec2 = UVec2{x: 10, y: 10};

pub struct TileSet
{
    texture: Texture2D,
    width: u16,
    height: u16
}

pub struct Tile
{
    x: u16,
    y: u16,
    index: u16,
    pub base_id: Option<UVec2>,
    pub top_id: Option<UVec2>,
}

pub struct TileMap
{
    width: usize,
    height: usize,
    tiles: Vec<Vec<SubMap>>
}

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
    fn new<F: Fn(usize, usize)->(Option<UVec2>, Option<UVec2>)>(width: u16, height: u16, x: usize, y: usize, gen: &F) -> Self
    {
        let mut tiles = Vec::with_capacity((width * height) as usize);
        for xi in 0..width
        {
            for yi in 0..height
            {
                let ids = gen(xi as usize + x, yi as usize + y);
                let index = yi * width + xi;
                tiles.push(Tile::new(xi as u16, yi as u16, index as u16, ids.0, ids.1));
            }
        }

        SubMap { width, height, x, y, tiles }
    }

    fn at(&self, x: u16, y: u16) -> &Tile
    {
        return &self.tiles[(y * self.width + x) as usize]
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

impl Tile
{
    fn new(x: u16, y: u16, index: u16, base_id: Option<UVec2>, top_id: Option<UVec2>) -> Self
    {
        Tile { x, y, index, base_id, top_id }
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
        if let Some(id) = self.top_id
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
        if let Some(id) = self.base_id
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

impl TileMap
{
    pub fn new<F: Fn(usize, usize)->(Option<UVec2>, Option<UVec2>)>(width: usize, height: usize, generator: &F) -> Self
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

                debug!("Generated mesh of size {}x{}", sub_width, sub_height);
                
                row.push(SubMap::new(sub_width, sub_height, x * SUB_MAP_MAX_SIZE.x as usize, y * SUB_MAP_MAX_SIZE.y as usize, generator));
            }
            tiles.push(row);
        }

        TileMap { width, height, tiles }
        
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    fn at(&self, x: usize, y: usize) -> &Tile
    {
        let map_x = x % SUB_MAP_MAX_SIZE.x as usize;
        let map_y = y % SUB_MAP_MAX_SIZE.y as usize;
        let map_index_x = (x / SUB_MAP_MAX_SIZE.x as usize) as u16;
        let map_index_y = (x / SUB_MAP_MAX_SIZE.y as usize) as u16;

        return &(self.tiles[map_y][map_x].at(map_index_x, map_index_y))
    }

    fn mut_at(&mut self, x: usize, y: usize) -> &Tile
    {
        let map_x = x % SUB_MAP_MAX_SIZE.x as usize;
        let map_y = y % SUB_MAP_MAX_SIZE.y as usize;
        let map_index_x = (x / SUB_MAP_MAX_SIZE.x as usize) as u16;
        let map_index_y = (x / SUB_MAP_MAX_SIZE.y as usize) as u16;

        return &mut (self.tiles[map_y][map_x].at(map_index_x, map_index_y))
    }
    
    pub fn to_mesh(&self, offset: Vec3, scale: f32, tileset: &TileSet) -> Vec<Mesh>
    {
        self.tiles.iter()
                  .flatten()
                  .map(|m| m.to_mesh(offset, scale, tileset))
                  .collect()
    }
}