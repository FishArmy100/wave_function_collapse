use core::fmt;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::BuildHasherDefault;

use itertools::Itertools;
use macroquad::prelude::*;
use crate::utils::Array2D;
use crate::utils::SliceDisplay;
use crate::wfc::*;
use crate::tile_set::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

#[derive(Clone, Debug)]
pub struct WFCEntity
{
    tiles: Vec<TileData>,
    tile_set: TileSet,
    wave: Wave<TileData>
}

impl WFCEntity
{
    pub fn new(model: &Array2D<TileData>, pattern_radius: usize, tile_set: TileSet, width: usize, height: usize, seed: u64) -> Self
    {
        let tiles_hash: HashSet<TileData, BuildHasherDefault<DefaultHasher>> = model.clone()
            .into_iter()
            .map(|(_, tile)| tile)
            .collect();

        let tiles = tiles_hash.into_iter().collect_vec();

        let patterns = Pattern::get_patterns(model, pattern_radius);

        let debug_patterns = (&patterns).into_iter().map(|p| p.center().clone()).collect::<Vec<TileData>>();
        println!("{}", SliceDisplay(&debug_patterns));

        let wave: Wave<TileData> = Wave::new(patterns, width, height, seed);
        WFCEntity { tiles, tile_set, wave }
    }

    pub fn step(&mut self) { self.wave.step() }

    pub fn get_mesh(&self, pos: Vec3, tile_size: f32) -> Vec<Mesh>
    {
        let grid = &self.wave.grid();
        TileMapEntity::new(pos, grid.width(), grid.height(), tile_size, self.tile_set.clone(), &|x, y| {
            match grid.at(x, y)
            {
                WaveTile::Collapsed(tile) => (tile.top, tile.base),
                WaveTile::SuperPos(_) => (None, None),
                WaveTile::Undefined => (None, None),
            }
        }).get_mesh()
    }

    pub fn width(&self) -> usize {self.wave.grid().width()}
    pub fn height(&self) -> usize {self.wave.grid().height()}
}