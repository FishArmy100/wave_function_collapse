use std::ops::Add;


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Array2D<T>
{
    width: usize,
    height: usize,
    data: Vec<T>
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct ArrayPos
{
    pub x: usize,
    pub y: usize
}

impl<T> Array2D<T> where T: Default + Clone
{
    pub fn new_default(width: usize, height: usize) -> Self
    {
        Array2D{width, height, data: vec![T::default(); width * height]}
    }
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

