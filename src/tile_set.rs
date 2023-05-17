use macroquad::prelude::*;
use serde::{Serialize, Deserialize, ser::SerializeMap};
use serde::de::*;

use futures::executor::block_on;

#[derive(Debug, Clone)]
pub struct TileSet
{
    texture: Texture2D,
    texture_path: String,
    width: u16,
    height: u16
}

impl TileSet
{
    pub async fn from_file(path: &str, width: u16, height: u16) -> Self
    {
        let texture = load_texture(path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        TileSet { texture, texture_path: String::from(path), width, height }
    }

    pub fn tile_count_width(&self) -> u16 {self.width}
    pub fn tile_count_height(&self) -> u16 {self.height}
    pub fn texture(&self) -> Texture2D {self.texture}

    pub fn tile_size(&self) -> (u16, u16) 
    {
        (
            self.texture.width() as u16 / self.tile_count_width(),
            self.texture.height() as u16 / self.tile_count_height()
        )
    }
}

impl Serialize for TileSet 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S : serde::Serializer 
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("width", &self.width)?;
        map.serialize_entry("height", &self.height)?;
        map.serialize_entry("texture_path", &self.texture_path)?;
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
        let mut width = None;
        let mut height = None;
        let mut texture_path = None;

        while let Some(key) = map.next_key::<&str>()?
        {
            if key == "width"
            {
                width = Some(map.next_value::<u16>()?)
            }
            else if key == "height"
            {
                height = Some(map.next_value::<u16>()?)
            }
            else if key == "texture_path"
            {
                texture_path = Some(map.next_value::<String>()?)
            }
            else
            {
                return Err(Error::custom(format!("Invalid key: {}", key)))
            }
        }

        if width.is_none() || height.is_none() || texture_path.is_none()
        {
            return Err(Error::custom(format!("Missing a value")))
        }
        
        let tileset = block_on(TileSet::from_file(texture_path.unwrap().as_str(), width.unwrap(), height.unwrap()));
        
        Ok(tileset)
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