use std::{ops::Add, fmt};


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
