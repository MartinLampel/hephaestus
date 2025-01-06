use macroquad::prelude::*;
use robot_simulation::environment::GridMap;
use robot_simulation::ui::grid_map_widget::GridMapWidget;
use robot_simulation::ui::robot_widget::RobotWidget;
use robot_simulation::ui::Widget;

use std::cell::RefCell;
use std::rc::Rc;


#[macroquad::main("Robot Simulator")]
async fn main() {
    
    let map = vec![vec![0u8,100, 100]];

    let grid_map = GridMap::new(100, 100, map);
    let mut grid_map_widget = GridMapWidget::new(&grid_map, 5.0, 5.0);      
    grid_map_widget.show_grid = true;

    let mut robot_widget = Rc::new(RefCell::new(RobotWidget::new(
        200.0, 200.0, 
        0.0)));
    let ui_widgets : Vec<Rc<RefCell<dyn Widget>>> = vec![Rc::new(RefCell::new(grid_map_widget)), 
    robot_widget.clone()];
    let mut speed = 0f32;
    let dt = 1.0;
    let mut last_time_stamp = get_time();
    loop {
        clear_background(BLACK);
            if get_time() - last_time_stamp > 0.05 {
                last_time_stamp = get_time();
                
                let mut r = robot_widget.borrow_mut();
            
                if is_key_down(KeyCode::Up) {
                    speed += 0.1;
                }

                if is_key_down(KeyCode::Down) {
                    speed -= 0.1;
                }

                if is_key_down(KeyCode::Left) {
                    r.angle -= 0.05;
                }

                if is_key_down(KeyCode::Right){
                    r.angle += 0.05;
                }            
            
                let delta_x = speed * dt * r.angle.cos();
                let delta_y = speed * dt *  r.angle.sin(); 

                r.x += delta_x;
                r.y += delta_y;                 
        }

        for element in &ui_widgets {
            element.borrow().render(); 
        }
        next_frame().await;
    }
     
}
