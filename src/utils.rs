use std::{ops::Add, fmt};
use macroquad::prelude::*;


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Array2D<T>
{
    width: usize,
    height: usize,
    data: Vec<T>
}

impl<T> Array2D<T>
{
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Array2D<T>
    {
        assert!(width * height == data.len());
        Array2D { width, height, data }
    }

    pub fn at(&self, x: usize, y: usize) -> &T
    {
        &self.data[y * self.width + x]
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut T
    {
        &mut self.data[y * self.width + x]
    }

    pub fn width(&self) -> usize {self.width}
    pub fn height(&self) -> usize {self.height}

    pub fn get_neighbors(&self, pos: ArrayPos, radius: usize) -> NeighborPositionIterator
    {
        NeighborPositionIterator::new(radius, self.width, self.height, pos)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NeighborPositionIterator
{
    x: isize,
    y: isize,
    radius: usize,
    array_width: usize,
    array_height: usize,
    start_pos: ArrayPos
}

impl NeighborPositionIterator
{
    pub fn new(radius: usize, width: usize, height: usize, start_pos: ArrayPos) -> Self
    {
        Self 
        { 
            x: (start_pos.x as isize - radius as isize + 1), 
            y: (start_pos.y as isize - radius as isize + 1),
            radius, 
            array_width: 
            width, 
            array_height: height, 
            start_pos 
        }
    }

    fn is_in_array(&self) -> bool
    {
        !(  self.x < 0 || self.x >= self.array_width as isize || 
            self.y < 0 || self.y >= self.array_height as isize)
    }

    fn increment(&mut self)
    {
        self.x += 1;
        if self.x > (self.start_pos.x + self.radius - 1) as isize
        {
            self.x = self.start_pos.x as isize - self.radius as isize + 1;
            self.y += 1;
        }
    }

    fn is_at_end(&self) -> bool
    {
        self.y > (self.start_pos.y as isize + self.radius as isize - 1)
    }
}

impl Iterator for NeighborPositionIterator
{
    type Item = ArrayPos;

    fn next(&mut self) -> Option<Self::Item> 
    {
        while !self.is_in_array() && !self.is_at_end()
        {
            self.increment();
        }

        if self.start_pos.x as isize == self.x && self.start_pos.y as isize == self.y
        {
            self.increment();
            while !self.is_in_array() && !self.is_at_end()
            {
                self.increment();
            }
        }

        if self.is_at_end() 
        {
            None
        }
        else 
        {
            let pos = ArrayPos::new(self.x as usize, self.y as usize);
            self.increment();
            Some(pos)
        }
    }
}


impl<T> Array2D<T> where T: Default + Clone
{
    pub fn new_default(width: usize, height: usize) -> Self
    {
        Array2D{width, height, data: vec![T::default(); width * height]}
    }
}

impl<T> fmt::Display for Array2D<T> where T : fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        if let Err(e) = f.write_str("[\n")
        {
            return Err(e);
        }

        for y in 0..self.height
        {
            if let Err(e) = f.write_str("\t[")
            {
                return Err(e);
            }

            for x in 0..self.width
            {
                if x != 0
                {
                    if let Err(e) = f.write_str(", ")
                    {
                        return Err(e);
                    }
                }
                if let Err(e) = write!(f, "{}", self.at(x, y))
                {
                    return Err(e);
                }
                
            }

            if let Err(e) = f.write_str("]\n")
            {
                return Err(e);
            }
        }

        if let Err(e) = f.write_str("]\n")
        {
            return Err(e);
        }

        Ok(())
    }
}

pub struct Array2DIter<'a, T>
{
    x: usize,
    y: usize,
    array: &'a Array2D<T>
}

impl<'a, T> IntoIterator for &'a Array2D<T> where T : Clone
{
    type Item = (ArrayPos, T);

    type IntoIter = Array2DIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter{x: 0, y: 0, array: &self}
    }
}

impl<'a, T> Iterator for Array2DIter<'a, T> where T: Clone
{
    type Item = (ArrayPos, T);
    fn next(&mut self) -> Option<Self::Item> {
        
        let next = if self.y >= self.array.height
        {
            None
        }
        else 
        {
            Some((ArrayPos{x: self.x, y: self.y}, self.array.at(self.x, self.y).clone()))
        };
        
        self.x += 1;
        if self.x >= self.array.width {self.y += 1; self.x = 0;}

        next
    }
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct ArrayPos
{
    pub x: usize,
    pub y: usize
}

impl ArrayPos
{
    pub fn new(x: usize, y: usize) -> ArrayPos
    {
        ArrayPos { x, y }
    }
}

impl Add for ArrayPos
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        ArrayPos{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl fmt::Display for ArrayPos
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct SliceDisplay<'a, T: 'a>(pub &'a [T]);

impl<'a, T: fmt::Display + 'a> fmt::Display for SliceDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for item in self.0 {
            if !first {
                write!(f, ",\n{}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        Ok(())
    }
}