

pub mod grid_map_widget;
pub mod robot_widget;

pub trait Widget {
    fn render(&self);
}


pub trait UIGrid : Widget {
    fn render_grid(&self);
}