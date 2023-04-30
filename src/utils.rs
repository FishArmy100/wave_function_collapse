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