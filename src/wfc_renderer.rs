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

#[derive(Clone, Debug)]
pub struct WFCEntity
{
    tiles: Vec<TileData>,
    wave: Wave<TileData>,
    entity: TileMapEntity
}

impl WFCEntity
{
    pub fn new(model: &Array2D<TileData>, pattern_radius: usize, tile_set: TileSet, pos: Vec3, tile_size: f32, width: usize, height: usize, seed: u64) -> Self
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
        let entity = TileMapEntity::new(pos, width, height, tile_size, tile_set, &|_, _| TileData::default());
        WFCEntity { tiles, wave, entity }
    }

    pub fn step(&mut self, step_count: usize) 
    { 
        for _ in 0..step_count
        {
            self.wave.step();
        }

        self.update_visual();
    }

    fn update_visual(&mut self)
    {
        for x in 0..self.width()
        {
            for y in 0..self.height()
            {
                let data = match self.wave.grid().at(x, y)
                {
                    WaveTile::Collapsed(tile) => TileData::new(tile.top, tile.base),
                    WaveTile::SuperPos(_) => TileData::default(),
                    WaveTile::Undefined => TileData::default(),
                };

                self.entity.set_without_update(x, y, data);
            }
        }

        self.entity.update();
    }

    pub fn render(&self)
    {
        self.entity.render();
    }

    pub fn width(&self) -> usize {self.wave.grid().width()}
    pub fn height(&self) -> usize {self.wave.grid().height()}
    pub fn tile_set(&self) -> &TileSet {self.entity.tile_map().tile_set()}
}