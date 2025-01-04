
use crate::robot::state::RobotState;
use crate::robot::state::Position;

pub struct ControlInput {
    pub velocity: f32,
    pub angular_velocity: f32,
}

impl ControlInput {
    pub fn new(velocity: f32, angular_velocity: f32) -> Self {
        Self {
            velocity,
            angular_velocity,
        }
    }
}

pub trait ControlAlgorithm {
    fn calculate_input(&self, state: &RobotState, dt: f32) -> ControlInput;
    fn set_target(&mut self, target: &Position);
}