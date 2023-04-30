use std::{hash::{Hash, BuildHasherDefault}, collections::{HashSet, hash_map::DefaultHasher}, fmt::{self, Debug}};
use itertools::Itertools;
use rand::prelude::*;
use crate::utils::{Array2D, ArrayPos};


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Pattern<T>
{
    radius: usize,
    tiles: Array2D<T>
}

fn is_in_grid<T>(pos: ArrayPos, radius: usize, grid: &Array2D<T>) -> bool 
    where T : Clone + PartialEq
{
    pos.x >= radius - 1 && pos.y >= radius - 1 && pos.x + radius - 1 < grid.width() && pos.y + radius - 1 < grid.height()
}

impl<T> fmt::Display for Pattern<T> where T : fmt::Display + Clone + Eq
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tiles)
    }
}

impl<T> Pattern<T> where T : Clone + Eq + Hash
{
    pub fn from_grid(pos: ArrayPos, radius: usize, grid: &Array2D<T>) -> Option<Pattern<T>>
    {
        assert!(radius != 0);

        if is_in_grid(pos, radius, grid)
        {
            let pattern_size = radius * 2 - 1;
            let mut data = Vec::<T>::with_capacity(pattern_size.pow(2));
            for y in (pos.y + 1 - radius)..(pos.y + radius)
            {
                for x in (pos.x + 1 - radius)..(pos.x + radius)
                {
                    data.push(grid.at(x, y).clone());
                }
            }

            Some(Self {radius, tiles: Array2D::<T>::new(pattern_size, pattern_size, data)})
        }
        else 
        {
            None
        }
    }

    pub fn get_patterns(grid: &Array2D<T>, radius: usize) -> Vec<Pattern<T>>
    {
        let mut patterns = vec![];
        for x in 0..grid.width()
        {
            for y in 0..grid.height()
            {
                if let Some(pattern) = Self::from_grid(ArrayPos::new(x, y), radius, grid)
                {
                    patterns.push(pattern);
                }
            }
        }

        let set : HashSet<_, BuildHasherDefault<DefaultHasher>> = patterns.drain(..).collect();
        patterns.extend(set.into_iter());

        patterns
    }

    pub fn data(&self) -> &Array2D<T> {&self.tiles}
    pub fn radius(&self) -> usize {self.radius}
    pub fn center(&self) -> &T {self.tiles.at(self.radius - 1, self.radius - 1)}
}

#[derive(Clone, Debug)]
pub enum WaveTile<T>
{
    Collapsed(T),
    SuperPos(Vec<usize>),
    Undefined,
}

impl<T> WaveTile<T>
{
    pub fn is_collapsed(&self) -> bool
    {
        match *self 
        {
            Self::Collapsed(_) => true,
            _ => false
        }
    }

    pub fn get_super_pos(&self) -> &Vec<usize>
    {
        match self 
        {
            WaveTile::SuperPos(p) => p,
            _ => panic!("Expected a superposition."),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Wave<T> where T : Clone
{
    patterns: Vec<Pattern<T>>,
    wave: Array2D<WaveTile<T>>,
    rng: StdRng
}

impl<T> Wave<T> where T : Eq + Hash + Clone
{
    pub fn grid(&self) -> &Array2D<WaveTile<T>> {&self.wave}

    pub fn new(patterns: Vec<Pattern<T>>, width: usize, height: usize, seed: u64) -> Self
    {
        let wave_tiles = WaveTile::SuperPos((0..patterns.len()).collect());
        let wave = Array2D::<WaveTile<T>>::new(width, height, vec![wave_tiles; width * height]);
        let rng = StdRng::seed_from_u64(seed);
        Wave { patterns, wave, rng}
    }

    pub fn collapse(&mut self, pos: ArrayPos)
    {
        let collapsed = if let WaveTile::SuperPos(possibilities) = self.wave.at_mut(pos.x, pos.y)
        {
            let index = self.rng.gen_range(0..possibilities.len());
            //println!("{}", index);
            possibilities[index]
        }
        else 
        {
            panic!("Tile is already collapsed")
        };

        *self.wave.at_mut(pos.x, pos.y) = WaveTile::<T>::Collapsed(self.patterns[collapsed].center().clone());
    }

    pub fn propagate(&mut self, pos: ArrayPos)
    {
        if let WaveTile::SuperPos(possibilities) = self.wave.at_mut(pos.x, pos.y)
        {
            let copy = possibilities.clone();
            // you could explicitly `drop(possibilities)` here for clarity, but it doesnt really matter
            for pattern_index in copy
            {
                if !self.check_pattern(pos, &self.patterns[pattern_index]) // this says it has already been borrowed mutably
                {
                    let WaveTile::SuperPos(possibilities) = self.wave.at_mut(pos.x, pos.y) else {unreachable!()}; // borrow possibilities again here
                    possibilities.retain(|&p| p != pattern_index)
                }
            }
        }
    }

    pub fn check_pattern(&self, pos: ArrayPos, pattern: &Pattern<T>) -> bool
    {
        for x in 0..pattern.radius * 2
        {
            for y in 0..pattern.radius * 2
            {
                let x = x as isize;
                let y = y as isize;

                let wave_x = x - pattern.radius as isize + 1 + pos.x as isize;
                let wave_y = y - pattern.radius as isize + 1 + pos.y as isize;

                if !self.is_in_wave(x, y) {continue;}

                let pattern_tile = pattern.tiles.at(x as usize, y as usize);
                let wave_tile = self.wave.at((wave_x + x) as usize, (wave_y + y) as usize);

                let matchable = match wave_tile
                {
                    WaveTile::Collapsed(state) => pattern_tile == state,
                    _ => true
                };

                if !matchable
                {
                    return false;
                }
            }
        }

        true
    }

    fn is_in_wave(&self, x: isize, y: isize) -> bool
    {
        x >= 0 && y >= 0 && x < self.wave.width() as isize && y < self.wave.height() as isize
    }

    pub fn observe(&self) -> Option<ArrayPos>
    {
        let tiles = self.wave.into_iter()
            .filter(|(_, tile)| matches!(tile, WaveTile::SuperPos(_)))
            .map(|(pos, tile)| (pos, tile.get_super_pos().len()))
            .sorted_by(|a, b| a.1.cmp(&b.1))
            .collect::<Vec<(ArrayPos, usize)>>();

        if tiles.len() > 0
        {
            Some(tiles[0].0)
        }
        else 
        {
            None
        }
    }

    pub fn step(&mut self)
    {
        if let Some(tile) = self.observe()
        {
            self.collapse(tile);
            self.propagate(tile);
        }
    }
}