use std::{hash::Hash, collections::HashSet, fmt};

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

        let set : HashSet<_> = patterns.drain(..).collect();
        patterns.extend(set.into_iter());

        patterns
    }

    pub fn data(&self) -> &Array2D<T> {&self.tiles}
}