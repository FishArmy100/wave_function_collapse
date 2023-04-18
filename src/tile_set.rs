use std::ops::{Add, Mul};

use macroquad::prelude::*;
use macroquad::models::{Vertex, Mesh};

pub struct TileSet
{
    pub texture: Texture2D,
    pub tile_size: f32,
    pub width: u16,
    pub height: u16
}

pub struct Tile
{
    pub x: usize,
    pub y: usize,
    pub id: u16,
    pub top_id: Option<u16>,
}

pub struct TileMap
{
    width: usize,
    height: usize,
    tiles: Vec<Tile>
}

struct TileInfo
{
    index: u16,
    x: u16,
    y: u16,
    id: u16,
    top_id: Option<u16>,
}

impl TileInfo
{
    fn new(x: u16, y: u16, index: u16, id: u16) -> Self
    {
        TileInfo { index, x, y, id, top_id: None }
    }

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

    fn get_verticies(&self, offset: Vec3, scale: f32) -> [Vertex; 4]
    {
        let gen = |xoffset, yoffset| 
        {
            let pos = Vec3{x: self.x as f32 + xoffset, y: self.y as f32 + yoffset, z: 0.0} * scale + offset;
            Vertex{position: pos, uv: Vec2::ZERO, color: GRAY}
        };

        [gen(0.0, 0.0), gen(1.0, 0.0), gen(0.0, 1.0), gen(1.0, 1.0)]
    }
}

fn get_index<T: Add<Output = T> + Mul<Output = T>>(x: T, y: T, width: T) -> T
{
    y * width + x
}

impl TileMap
{
    pub fn new<F: Fn(usize, usize)->Tile>(width: usize, height: usize, generator: F) -> Self
    {
        let mut tiles = Vec::with_capacity(width * height);
        for x in 0..width
        {
            for y in 0..height
            {
                tiles.push(generator(x, y));
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
    
    pub fn to_mesh(&self, offset: Vec3, scale: f32) -> Mesh
    {
        let mut verticies = Vec::with_capacity(self.height * self.width * 4);
        let mut triangles = Vec::with_capacity(self.width * self.height * 6);
        let mut index = 0;

        for x in 0..self.width - 1
        {
            for y in 0..self.height - 1
            {
                let info = TileInfo::new(x as u16, y as u16, index, self.at(x, y).id);
                verticies.extend_from_slice(&info.get_verticies(offset, scale));
                triangles.extend_from_slice(&info.get_triangles());
                index += 1;
            }
        }

        Mesh { vertices: verticies, indices: triangles, texture: None }
    }
}