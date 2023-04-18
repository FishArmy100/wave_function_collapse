use std::ops::{Add, Mul};

use macroquad::prelude::*;
use macroquad::models::{Vertex, Mesh};

const SUB_MAP_MAX_SIZE: UVec2 = UVec2{x: 25, y: 25};

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
    pub id: UVec2,
    pub top_id: Option<UVec2>,
}

pub struct TileMap
{
    width: usize,
    height: usize,
    tiles: Vec<Tile>
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
    fn new<F: Fn(usize, usize)->(UVec2, Option<UVec2>)>(width: u16, height: u16, x: usize, y: usize, gen: F)
    {
        let mut tiles = Vec::with_capacity((width * height) as usize);
        for xi in 0..width
        {
            for yi in 0..height
            {
                let ids = gen(xi as usize + x, yi as usize + y);
                tiles.push(Tile::new(x as u16, y as u16, index as u16, ids.0, ids.1));
            }
        }
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
    fn new(x: u16, y: u16, index: u16, id: UVec2, top_id: Option<UVec2>) -> Self
    {
        Tile { x, y, index, id, top_id }
    }

    pub fn x(&self) -> u16 {self.x}
    pub fn y(&self) -> u16 {self.y}
    pub fn index(&self) -> u16 {self.index}

    fn get_triangles(&self) -> [u16; 6]
    {
        let base_index = self.index * 4;
        [
            base_index,
            base_index + 1,
            base_index + 2,

            base_index + 1,
            base_index + 3,
            base_index + 2
        ]
    }

    fn get_verticies(&self, offset: Vec3, scale: f32, tileset: &TileSet) -> [Vertex; 4]
    {
        let gen = |xoffset, yoffset| 
        {
            let tile_size = Vec2{x: tileset.texture.width() / tileset.width as f32, y: tileset.texture.height() / tileset.height as f32};
            let uv_pos = Vec2{x: tile_size.x * (self.id.x as f32 + xoffset), y: tile_size.y * (self.id.y as f32 + yoffset)};
            let uv = Vec2{x: uv_pos.x / tileset.texture.width(), y: uv_pos.y / tileset.texture.height()};

            let pos = Vec3{x: self.x as f32 + xoffset, y: self.y as f32 + yoffset, z: 0.0} * scale + offset;
            Vertex{position: pos, uv, color: WHITE}
        };

        [gen(0.0, 0.0), gen(1.0, 0.0), gen(0.0, 1.0), gen(1.0, 1.0)]
    }
}

impl TileMap
{
    pub fn new<F: Fn(usize, usize)->(UVec2, Option<UVec2>)>(width: usize, height: usize, generator: F) -> Self
    {
        let mut tiles = Vec::with_capacity(width * height);
        for x in 0..width
        {
            for y in 0..height
            {
                let index = y * width + x;
                let ids = generator(x, y);
                tiles.push(Tile::new(x as u16, y as u16, index as u16, ids.0, ids.1));
            }
        }

        Self { width, height, tiles }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn at(&self, x: usize, y: usize) -> &Tile
    {
        return &self.tiles[y * self.width + x]
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Tile
    {
        return &mut self.tiles[y * self.width + x]
    }
    
    pub fn to_mesh(&self, offset: Vec3, scale: f32, tileset: &TileSet) -> Mesh
    {
        let mut verticies = Vec::with_capacity(self.height * self.width * 4);
        let mut triangles = Vec::with_capacity(self.width * self.height * 6);

        for x in 0..self.width
        {
            for y in 0..self.height
            {
                verticies.extend_from_slice(&self.at(x, y).get_verticies(offset, scale, tileset));
                triangles.extend_from_slice(&self.at(x, y).get_triangles());
            }
        }

        Mesh { vertices: verticies, indices: triangles, texture: Some(tileset.texture) }
    }
}