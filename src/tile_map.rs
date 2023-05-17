use core::fmt;

use itertools::Itertools;
use macroquad::prelude::*;
use macroquad::models::{Vertex, Mesh};

use serde::{Serialize, Deserialize};

use crate::utils::{Array2D, ArrayPos};
use crate::tile_set::*;

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
    pub data: Option<usize>
}

impl Tile
{
    fn new(x: u16, y: u16, data: Option<usize>) -> Self
    {
        Tile { x, y, data }
    }

    pub fn x(&self) -> u16 {self.x}
    pub fn y(&self) -> u16 {self.y}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMap
{
    width: usize,
    height: usize,
    tiles: Vec<TileData>,
    map: Array2D<Tile>,
    tile_set: TileSet,
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
        let map = Array2D::<Tile>::new(width, height, &|x, y| 
        {
            let data = generator(x, y);
            Tile::new(x as u16, y as u16, data)
        });

        TileMap { width, height, map, tiles, tile_set }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn tile_set(&self) -> &TileSet {&self.tile_set}
    pub fn tiles(&self) -> &Vec<TileData> {&self.tiles}

    pub fn at(&self, x: usize, y: usize) -> &Tile
    {
        return self.map.at(x, y);
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Tile
    {
        return self.map.at_mut(x, y);
    }

    fn set_sub_image(&self, image: &mut Image, offset: ArrayPos, tile_set_image: &Image)
    {
        let tile = self.at(offset.x, offset.y);
        let tile_size = self.tile_set().tile_size();
        let start_pos = (tile_size.0 * offset.x as u16, tile_size.1 * offset.y as u16);

        let Some(index) = tile.data else { return; };
        let tile_data = &self.tiles[index];

        for x in 0..tile_size.0
        {
            for y in 0..tile_size.1
            {
                let index_x = offset.x + x as usize;
                let index_y = offset.y + y as usize;

                let mut pixel_color = image.get_pixel(index_x as u32, index_y as u32);
                for texture_index in &tile_data.textures
                {
                    let pixel_pos = (texture_index.x * tile_size.0 + x, texture_index.y * tile_size.1 + y);
                    let tile_pixel = tile_set_image.get_pixel(pixel_pos.0 as u32, pixel_pos.1 as u32);
                    pixel_color = layer_color(tile_pixel, pixel_color);
                }

                image.set_pixel((start_pos.0 + x) as u32, (start_pos.1 + y) as u32, pixel_color)
            }
        }
    }

    pub fn get_image(&self, background_color: Color) -> Image
    {
        let tile_size = self.tile_set.tile_size();
        let tile_set_image = self.tile_set.texture().get_texture_data();

        let mut image = Image::gen_image_color(tile_size.0 * self.width() as u16, tile_size.1 * self.height() as u16, background_color);

        for item in &self.map
        {
            let (pos, _) = item;
            self.set_sub_image(&mut image, pos, &tile_set_image)
        }

        image
    }

    pub fn get_texture(&self, background_color: Color, filter_mode: FilterMode) -> Texture2D
    {
        let texture = Texture2D::from_image(&self.get_image(background_color));
        texture.set_filter(filter_mode);
        texture
    }
}

