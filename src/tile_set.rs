use macroquad::prelude::*;
use serde::{Serialize, Deserialize, ser::SerializeMap};
use serde::de::*;

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

impl Serialize for TileSet 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S : serde::Serializer 
    {
        let bytes = self.texture.get_texture_data().bytes;
        let bytes_string = String::from_utf8_lossy(&bytes);

        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("width", &self.width)?;
        map.serialize_entry("height", &self.height)?;
        map.serialize_entry("texture_width", &self.texture.width())?;
        map.serialize_entry("texture_height", &self.texture.height())?;
        map.serialize_entry("texture_data", &bytes_string)?;
        map.end()
    }
}

struct TileSetVisitor;

impl<'de> Visitor<'de> for TileSetVisitor
{
    type Value = TileSet;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a TileSet")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where M : MapAccess<'de>, 
    {
        let width = map.next_value()?;
        let height = map.next_value()?;
        let texture_width = map.next_value()?;
        let texture_height = map.next_value()?;

        let bytes_string = map.next_value::<String>()?;
        let data = bytes_string.as_bytes();

        let texture = Texture2D::from_rgba8(texture_width, texture_height, &data);

        Ok(TileSet {texture, width, height})
    }
}

impl<'de> Deserialize<'de> for TileSet
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D : serde::Deserializer<'de> 
    {
        deserializer.deserialize_map(TileSetVisitor{})
    }
} 