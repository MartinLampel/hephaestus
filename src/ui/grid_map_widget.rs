

use macroquad::{color::WHITE, color::LIGHTGRAY, shapes::{draw_line, draw_rectangle}};

use crate::environment::GridMap;

use super::{Widget, UIGrid};

pub struct GridMapWidget<'a> {
    pub grid_map: &'a GridMap,
    pub cell_width: f32,
    pub cell_height: f32,
    pub resolution: (f32, f32),
    pub show_grid: bool,
}


impl <'a> GridMapWidget<'a> {
    pub fn new(grid_map: &'a GridMap, cell_width: f32, cell_height: f32) -> Self {
        let resolution = (
            grid_map.width as f32 * cell_width,
            grid_map.height as f32 * cell_height,
        );
        let show_grid = false;

        Self {
            grid_map,
            cell_width,
            cell_height,
            resolution,
            show_grid
        }
    }
}

impl <'a> Widget for GridMapWidget<'a> {

    fn render(&self) {

        draw_rectangle(0.0, 0.0, self.resolution.0, self.resolution.1, WHITE);
        self.render_grid();
        
      
    }
}

impl <'a> UIGrid for GridMapWidget<'a> {
    fn render_grid(&self) {
        if self.show_grid {
            for i in 0..self.grid_map.width {
                draw_line(
                    i as f32 * self.cell_width,
                    0.0,
                    i as f32 * self.cell_width,
                    self.resolution.1,
                    1.0,
                    LIGHTGRAY,
                );
            }

            for i in 0..self.grid_map.height {
                draw_line(
                    0.0,
                    i as f32 * self.cell_height,
                    self.resolution.0,
                    i as f32 * self.cell_height,
                    1.0,
                    LIGHTGRAY,
                );
            }
        }
    }
    
}