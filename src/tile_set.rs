use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct TileSet
{
    texture: Texture2D,
    width: u16,
    height: u16
}

impl TileSet
{
    pub fn new(texture: Texture2D, width: u16, height: u16) -> Self
    {
        TileSet { texture, width, height}
    }

    pub fn tile_count_width(&self) -> u16 {self.width}
    pub fn tile_count_height(&self) -> u16 {self.height}
    pub fn texture(&self) -> Texture2D {self.texture}

    fn get_tile_uv(&self, x: u16, y: u16) -> Vec2
    {
        let tile_size = Vec2{x: self.texture.width() / self.tile_count_width() as f32, y: self.texture.height() / self.tile_count_height() as f32};
        let uv_pos = Vec2{x: tile_size.x * (x as f32), y: tile_size.y * (y as f32)};
        let uv = Vec2{x: uv_pos.x / self.texture.width(), y: uv_pos.y / self.texture.height()};
        uv
    }

    pub fn get_tile_uvs(&self, x: u16, y: u16) -> [[Vec2; 2]; 2]
    {
        assert!(x < self.width && y < self.height, "Index was out of range of the tileset texture");
        [
            [self.get_tile_uv(x, y), self.get_tile_uv(x + 1, y)],
            [self.get_tile_uv(x, y + 1), self.get_tile_uv(x + 1, y + 1)],
        ]
    }
}