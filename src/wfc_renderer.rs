use macroquad::prelude::*;
use crate::utils::Array2D;
use crate::wfc::*;
use crate::tile_set::*;

#[derive(Clone, Debug)]
pub struct WFCEntity
{
    wave: Wave<TileData>,
    entity: TileMapEntity,
    error_tile: TileData
}

impl WFCEntity
{
    pub fn new(model: &Array2D<TileData>, pattern_radius: usize, tile_set: TileSet, pos: Vec3, tile_size: f32, width: usize, height: usize, seed: u64, error_tile: TileData) -> Self
    {
        let wave: Wave<TileData> = Wave::new(model, pattern_radius, width, height, seed);
        let entity = TileMapEntity::new(pos, width, height, tile_size, tile_set, &|_, _| TileData::default());
        WFCEntity { wave, entity, error_tile }
    }

    pub fn step(&mut self, step_count: usize) 
    { 
        for _ in 0..step_count
        {
            self.wave.step();
        }

        self.update_visual();
    }

    pub fn collapse_full(&mut self)
    {
        self.wave.collapse_full();
        self.update_visual();
    }

    fn update_visual(&mut self)
    {
        for x in 0..self.width()
        {
            for y in 0..self.height()
            {
                let data = match self.wave.grid().at(x, y)
                {
                    WaveTile::Collapsed(tile) => TileData::new(tile.top, tile.base),
                    WaveTile::SuperPos(_) => TileData::default(),
                    WaveTile::Undefined => self.error_tile,
                };

                self.entity.set_without_update(x, y, data);
            }
        }

        self.entity.update();
    }

    pub fn render(&self)
    {
        self.entity.render();
        self.render_text_info();
        self.render_debug_lines();
    }

    fn render_debug_lines(&self)
    {
        let line_color = BLACK;
        let line_thickness = 3.0;

        let pos = self.entity.pos;
        let size = vec2(self.width() as f32 * self.tile_size(), self.height() as f32 * self.tile_size());
        
        // draw outline
        draw_rectangle_lines(pos.x, pos.y, size.x, size.y, line_thickness * 2.0, line_color);

        // vertical lines
        for x in 1..self.width()
        {
            let x_pos = pos.x + self.tile_size() * x as f32;
            draw_line(x_pos, pos.y, x_pos, pos.y + size.y, line_thickness, line_color);
        }

        // horizontal lines
        for y in 1..self.height()
        {
            let y_pos = pos.y + self.tile_size() * y as f32;
            draw_line(pos.x, y_pos, pos.x + size.x, y_pos, line_thickness, line_color);
        }
    }

    fn render_text_info(&self) {
        for x in 0..self.width()
        {
            for y in 0..self.height()
            {
                let text = match self.wave.grid().at(x, y)
                {
                    WaveTile::Collapsed(_) => None,
                    WaveTile::SuperPos(pos) => Some(format!("{}", pos.len())),
                    WaveTile::Undefined => None,
                };

                if let Some(text) = text
                {
                    let x = x as f32 * self.tile_size() + self.entity.pos.x + self.tile_size() / 2.0;
                    let y = y as f32 * self.tile_size() + self.entity.pos.y + self.tile_size() / 2.0;
                    draw_text(&text, x, y, self.tile_size() / 2.0, WHITE)
                }
            }
        }
    }

    pub fn width(&self) -> usize {self.wave.grid().width()}
    pub fn height(&self) -> usize {self.wave.grid().height()}
    pub fn tile_size(&self) -> f32 {self.entity.tile_size}
    pub fn tile_set(&self) -> &TileSet {self.entity.tile_map().tile_set()}
}