

use crate::robot::state::RobotState;
use crate::robot::control::ControlInput;

pub trait KinematicsModel {
    fn predict_state(
        &self,
        current_state: &RobotState,
        control_input: &ControlInput,
        dt: f32,
    ) -> RobotState;
}


pub struct DifferentialDrive;

impl KinematicsModel for DifferentialDrive {
    fn predict_state(
        &self,
        current_state: &RobotState,
        control_input: &ControlInput,
        dt: f32,
    ) -> RobotState {
        let v = control_input.velocity;
        let omega = control_input.angular_velocity;

        let delta_x = v * dt * current_state.orientation.cos();
        let delta_y = v * dt * current_state.orientation.sin();
        let delta_theta = omega * dt;

        let mut new_state = current_state.clone();
        new_state.position.translate(delta_x, delta_y);
        new_state.orientation += delta_theta;

        new_state
    }
}