
use super::{UIElement};

use macroquad::{color::{BLACK, BLUE}, math::RectOffset, shapes::{draw_circle, draw_line, draw_rectangle}};

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

impl UIElement for RobotWidget {
    fn render(&self) {
        draw_circle(self.x, self.y, self.radius, BLUE);

        let offsets = [-self.radius - self.wheel_width / 2.0, self.radius + self.wheel_width / 2.0];
       
        let orientation = self.angle;// + std::f32::consts::FRAC_PI_2;
        for offset in offsets {
            let x_start = self.x + (self.wheel_length/2.0  * f32::cos(self.angle) - offset * f32::sin(self.angle));
            let y_start = self.y + (self.wheel_length/2.0  * f32::sin(self.angle) + RectOffset * f32::cos(self.angle));
            draw_line(
                x_start, y_start, 
                x_start - self.wheel_length * f32::cos(orientation), 
                y_start - self.wheel_length * f32::sin(orientation), 
                self.wheel_width, 
                BLACK);       
        }
       
 
    }
}