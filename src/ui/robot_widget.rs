
use super::{Widget};

use macroquad::{color::BLUE, color::BLACK, color::GOLD, shapes::{draw_line, draw_poly, draw_circle}};

const RAD_TO_DEG: f32 = 180.0/std::f32::consts::PI;

pub struct RobotWidget {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub radius: f32,
    pub wheel_length: f32,
    pub wheel_width: f32,
}


impl RobotWidget {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Self {
            x,
            y,

            angle,
            radius: 10.0,
            wheel_length: 10.0,
            wheel_width: 4.0,
        }
    }
}

impl Widget for RobotWidget {
    fn render(&self) {
        draw_circle(self.x, self.y, self.radius, BLUE);
        draw_poly(self.x, self.y, 3, 5.0, self.angle * RAD_TO_DEG, GOLD);

        let offsets = [-self.radius - self.wheel_width / 2.0, self.radius + self.wheel_width / 2.0];
       
        let orientation = self.angle;
        for offset in offsets {
            let x_start = self.x + (self.wheel_length/2.0  * f32::cos(self.angle) - offset * f32::sin(self.angle));
            let y_start = self.y + (self.wheel_length/2.0  * f32::sin(self.angle) + offset * f32::cos(self.angle));
            draw_line(
                x_start, y_start, 
                x_start - self.wheel_length * f32::cos(orientation), 
                y_start - self.wheel_length * f32::sin(orientation), 
                self.wheel_width, 
                BLACK);       
        }
       
 
    }
}