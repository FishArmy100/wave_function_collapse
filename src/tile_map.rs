use core::fmt;

use itertools::Itertools;
use macroquad::prelude::*;
use macroquad::models::{Vertex, Mesh};

use serde::{Serialize, Deserialize};

use crate::utils::{Array2D, ArrayPos};
use crate::tile_set::*;

const SUB_MAP_MAX_SIZE: UVec2 = UVec2{x: 10, y: 10};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn new<F: Fn(usize, usize)->Option<usize>>(width: u16, height: u16, x: usize, y: usize, gen: &F) -> Self
    {
        let mut tiles = vec![Tile{x: 0, y: 0, index: 0, data: None}; (width * height) as usize];
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

    fn to_mesh(&self, offset: Vec3, scale: f32, tileset: &TileSet, tiles: &Vec<TileData>) -> Mesh
    {
        let mut verticies = Vec::with_capacity(self.height as usize * self.width as usize * 4);
        let mut triangles = Vec::with_capacity(self.height as usize * self.width as usize * 6);
        let vertex_offset = offset + Vec3{x: self.x as f32, y: self.y as f32, z: 0.0} * scale;
        let mut triangle_offset = 0;
        for x in 0..self.width
        {
            for y in 0..self.height
            {
                let generated_verticies = self.at(x, y).get_verticies(vertex_offset, scale, tileset, tiles);
                for verts in generated_verticies
                {
                    verticies.extend_from_slice(&verts);
                    triangles.extend_from_slice(&self.at(x, y).get_triangles(triangle_offset));
                    triangle_offset += 4;
                }
                
            }
        }

        Mesh { vertices: verticies, indices: triangles, texture: Some(tileset.texture()) }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Default, Serialize, Deserialize)]
pub struct TileIndex
{
    pub x: u16,
    pub y: u16,
}

impl TileIndex
{
    pub fn new(x: u16, y: u16) -> TileIndex
    {
        TileIndex {x, y}
    }
}

pub fn tile_index(x: u16, y: u16) -> TileIndex
{
    TileIndex { x, y }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, Serialize, Deserialize)]
pub struct TileData
{
    pub name: String,
    pub debug_name: String,
    pub textures: Vec<TileIndex>,
}

impl fmt::Display for TileData
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = self.textures.iter().map(|t| format!("({}, {})", t.x, t.y)).join("; ");
        write!(f, "[{}]", str)
    }
}

impl TileData
{
    pub fn new(textures: Vec<TileIndex>, name: &str, debug_name: &str) -> TileData
    {
        TileData { textures, name: name.to_owned(), debug_name: debug_name.to_owned() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile
{
    x: u16,
    y: u16,
    index: u16,
    pub data: Option<usize>
}

impl Tile
{
    fn new(x: u16, y: u16, index: u16, data: Option<usize>) -> Self
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

    fn get_verticies_single(&self, index: TileIndex, offset: Vec3, scale: f32, tileset: &TileSet, tiles: &Vec<TileData>) -> [Vertex; 4]
    {
        let uvs = &tileset.get_tile_uvs(index.x, index.y);
        let gen = |xoffset, yoffset|
        {
            let pos = Vec3{x: self.x as f32 + xoffset as f32, y: self.y as f32 + yoffset as f32, z: 0.0} * scale + offset;
            Vertex{position: pos, uv: uvs[yoffset as usize][xoffset as usize], color: WHITE}
        };

        [gen(0, 0), gen(1, 0), gen(0, 1), gen(1, 1)]
    }

    pub fn get_verticies(&self, offset: Vec3, scale: f32, tileset: &TileSet, tiles: &Vec<TileData>) -> Vec<[Vertex; 4]>
    {
        let Some(index) = self.data else { return vec!() };
        let textures = &tiles[index].textures;
        let mut verticies: Vec<[Vertex; 4]> = Vec::with_capacity(textures.len());
        for texture in textures
        {
            verticies.push(self.get_verticies_single(*texture, offset, scale, tileset, tiles));
        }

        verticies
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMap
{
    width: usize,
    height: usize,
    tiles: Vec<TileData>,
    map: Vec<Vec<SubMap>>,
    tile_set: TileSet,
    sub_maps_x: usize,
    sub_maps_y: usize,
}

fn layer_color(top: Color, base: Color) -> Color
{
    let a = top.to_vec();
    let b = base.to_vec();

    let a_alpha = a.w;
    let b_alpha = b.w;

    let alpha = a_alpha + b_alpha * (1.0 - a_alpha);
    let a_color = a.truncate();
    let b_color = b.truncate();

    let o_color = (a_color * a_alpha + b_color * b_alpha * (1.0 - a_alpha)) / alpha;
    Color::from_vec(o_color.extend(alpha))
}

impl TileMap
{
    pub fn new<F: Fn(usize, usize)->Option<usize>>(width: usize, height: usize, tile_set: TileSet, tiles: Vec<TileData>, generator: &F) -> Self
    {
        let sub_maps_x = width / SUB_MAP_MAX_SIZE.x as usize + (if width % SUB_MAP_MAX_SIZE.x as usize != 0 {1} else {0});
        let sub_maps_y = height / SUB_MAP_MAX_SIZE.y as usize + (if height % SUB_MAP_MAX_SIZE.y as usize != 0 {1} else {0});

        let mut map = Vec::with_capacity(height);
        
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
            map.push(row);
        }

        TileMap { width, height, map, tiles, tile_set, sub_maps_x, sub_maps_y }
        
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn tile_set(&self) -> &TileSet {&self.tile_set}
    pub fn tiles(&self) -> &Vec<TileData> {&self.tiles}

    pub fn at(&self, x: usize, y: usize) -> &Tile
    {
        let map_x = x / SUB_MAP_MAX_SIZE.x as usize;
        let map_y = y / SUB_MAP_MAX_SIZE.y as usize;

        let map_index_x = (x % SUB_MAP_MAX_SIZE.x as usize) as u16;
        let map_index_y = (y % SUB_MAP_MAX_SIZE.y as usize) as u16;

        return self.map[map_y][map_x].at(map_index_x, map_index_y)
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Tile
    {
        let map_x = x / SUB_MAP_MAX_SIZE.x as usize;
        let map_y = y / SUB_MAP_MAX_SIZE.y as usize;

        let map_index_x = (x % SUB_MAP_MAX_SIZE.x as usize) as u16;
        let map_index_y = (y % SUB_MAP_MAX_SIZE.y as usize) as u16;

        return self.map[map_y][map_x].at_mut(map_index_x, map_index_y)
    }
    
    pub fn to_mesh(&self, offset: Vec3, scale: f32) -> Vec<Mesh>
    {
        self.map.iter()
                  .flatten()
                  .map(|m| m.to_mesh(offset, scale, &self.tile_set, &self.tiles))
                  .collect()
    }

    fn set_sub_image(&self, image: &mut Image, offset: ArrayPos, tile_set_image: &Image, background_color: Color)
    {
        let tile = self.at(offset.x, offset.y);
        let tile_size = self.tile_set().tile_size();
        let start_pos = (tile_size.0 * offset.x as u16, tile_size.1 * offset.y as u16);

        for x in 0..tile_size.0
        {
            for y in 0..tile_size.1
            {
                let mut pixel_color = background_color;
                for i in &tile.textures
                {
                    let pixel_pos = (i.x * tile_size.0 + x, i.y * tile_size.1 + y);
                    let tile_pixel = tile_set_image.get_pixel(pixel_pos.0 as u32, pixel_pos.1 as u32);
                    pixel_color = layer_color(tile_pixel, pixel_color);
                }

                image.set_pixel((start_pos.0 + x) as u32, (start_pos.1 + y) as u32, pixel_color)
            }
        }
    }

    pub fn gen_image(&self) -> Image
    {
        let tile_size = tileset.tile_size();
        let tileset_image = tileset.texture().get_texture_data();

        let mut image = Image::gen_image_color(tile_size.0 * tile_indexes.width() as u16, tile_size.1 * tile_indexes.height() as u16, color_u8!(0, 0, 0, 0));

        for item in tile_indexes
        {
            let (pos, tile) = item;
            set_sub_image(&mut image, &tiles[*tile], &tileset_image, tile_size, pos, BLACK);
        }

        image
    }
}

