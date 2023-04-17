
pub struct Tile
{
    x: usize,
    y: usize,
    id: u16,
    top_id: Option<u16>,
}

pub struct TileSet
{
    width: usize,
    height: usize,
    tiles: Vec<Tile>
}

impl TileSet
{
    pub fn new(width: usize, height: usize, generator: Box<dyn Fn(usize, usize)->Tile>) -> Self
    {
        let mut tiles = Vec::with_capacity(width * height);
        for x in 0..width - 1
        {
            for y in 0..height - 1
            {
                tiles.push(generator(x, y));
            }
        }

        Self { width, height, tiles }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn at(&self, x: usize, y: usize) -> &Tile
    {
        return &self.tiles[y * self.width + x]
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Tile
    {
        return &mut self.tiles[y * self.width + x]
    }
}