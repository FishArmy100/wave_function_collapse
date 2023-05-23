use crate::tile_map::TileData;
use crate::tile_set::TileSet;
use crate::utils::ArrayPos;
use crate::wfc::Pattern;
use macroquad::prelude::*;
use macroquad::ui::*;
use macroquad::ui::widgets::*;


#[derive(Debug, Clone)]
pub struct PatternViewer<'p>
{
    tiles: &'p Vec<TileData>,
    patterns: &'p Vec<Pattern<usize>>,
    tileset: &'p TileSet,
    texture: Texture2D,
    index: usize,
}

fn layer_color(top: Color, base: Color) -> Color
{
    let a = top.to_vec();
    let b = base.to_vec();

    let a_alpha = a.w;
    let b_alpha = b.w;

    let alpha = a_alpha + b_alpha * (1.0 - a_alpha);
    let a_color = a.truncate();
    let b_color = b.truncate();

    let o_color = (a_color * a_alpha + b_color * b_alpha * (1.0 - a_alpha)) / alpha;
    Color::from_vec(o_color.extend(alpha))
}

fn set_sub_image(image: &mut Image, tile: &TileData, tileset: &Image, tile_size: (u16, u16), offset: ArrayPos, background_color: Color)
{
    let start_pos = (tile_size.0 * offset.x as u16, tile_size.1 * offset.y as u16);

    for x in 0..tile_size.0
    {
        for y in 0..tile_size.1
        {
            let mut pixel_color = background_color;
            for i in &tile.textures
            {
                let pixel_pos = (i.x * tile_size.0 + x, i.y * tile_size.1 + y);
                let tile_pixel = tileset.get_pixel(pixel_pos.0 as u32, pixel_pos.1 as u32);
                pixel_color = layer_color(tile_pixel, pixel_color);
            }

            image.set_pixel((start_pos.0 + x) as u32, (start_pos.1 + y) as u32, pixel_color)
        }
    }
}

fn gen_image(tiles: &Vec<TileData>, pattern: &Pattern<usize>, tileset: &TileSet) -> Image
{
    let tile_size = tileset.tile_size();
    let pattern_size = (pattern.radius() * 2 - 1) as u16;
    let tileset_image = tileset.texture().get_texture_data();

    let mut image = Image::gen_image_color(tile_size.0 * pattern_size, tile_size.1 * pattern_size, color_u8!(0, 0, 0, 0));

    for item in pattern.data()
    {
        let (pos, tile) = item;
        set_sub_image(&mut image, &tiles[*tile], &tileset_image, tile_size, pos, BLACK);
    }

    image
}

impl<'p> PatternViewer<'p>
{
    pub fn new(tiles: &'p Vec<TileData>, patterns: &'p Vec<Pattern<usize>>, tileset: &'p TileSet) -> Self
    {
        assert!(patterns.len() > 0, "There must be alteast one pattern");
        let image = gen_image(tiles, &patterns[0], tileset);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        Self { tiles, patterns, tileset, texture, index: 0 }
    }

    fn update_texture(&mut self)
    {
        let image = gen_image(&self.tiles, &self.patterns[self.index], &self.tileset);
        self.texture = Texture2D::from_image(&image);
        self.texture.set_filter(FilterMode::Nearest);
    }

    pub fn update(&mut self) -> bool
    {
        let size = Vec2 {x: 300., y: 400.};
        let pos = Vec2 {x: 200., y: 200.};
        Window::new(hash!(), pos, size)
            .label("Pattern Viewer")
            .close_button(true)
            .ui(&mut root_ui(), |ui| 
            {
                let str = format!("pattern: {}/{}", self.index + 1, self.patterns.len());
                ui.label(None, &str);
                ui.texture(self.texture, size.x - 5., size.x - 5.);
                if self.index > 0
                {
                    if ui.button(None, "Previous")
                    {
                        self.index -= 1;
                    }
                    
                    self.update_texture(); 
                }
                if self.index < self.patterns.len() - 1
                {
                    ui.same_line(0.0);
                    if ui.button(None, "Next")
                    {
                        self.index += 1;
                    }

                    self.update_texture(); 
                }
            })
    }
}