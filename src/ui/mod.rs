

pub mod grid_map_widget;
pub mod robot_widget;

pub trait UIElement {
    fn render(&self);
}


pub trait UIGrid : UIElement {
    fn render_grid(&self);
}