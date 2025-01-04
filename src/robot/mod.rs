

pub mod state;
pub mod sensors;
pub mod control;
pub mod kinematics;

pub use state::RobotState;
pub use sensors::Sensor;
pub use control::ControlAlgorithm;
pub use kinematics::KinematicsModel;

use crate::prelude::*;

pub struct Robot {
    pub sensors: Vec<Box<dyn Sensor>>,
    pub control: Box<dyn ControlAlgorithm>,
    pub localization: Box<dyn LocalizationAlgorithm>,
    pub planner: Box<dyn PathPlanner>,
    pub kinematics: Box<dyn KinematicsModel>
}

impl Robot {

    pub fn with_sensor(mut self, sensor: Box<dyn Sensor>) -> Self {
        self.sensors.push(sensor);
        self
    }

    
    pub fn update(&mut self, dt: f32, state: &mut RobotState, environment: &Environment) {

        let mut measurements = Vec::new();
        for sensor in &self.sensors {
            measurements.push(sensor.measure(state, environment));
        }

        *state = self.localization.update(state, &measurements, dt);

        let control_input = self.control.calculate_input(state, dt);
        
        *state = self.kinematics.predict_state(state, &control_input, dt);

        state.odometry.update(control_input.velocity * dt, 0.0, control_input.angular_velocity * dt);

        state.velocity.linear = control_input.velocity;
        state.velocity.angular = control_input.angular_velocity;
       
    }

    pub fn plan_path_and_execute(&mut self, goal: Position, state: &mut RobotState, environment: &Environment) {
        let path = self.planner.plan_path(&state.position, &goal, environment);

        println!("Planned path: {:?}", path);

        for waypoint in path {
                self.control.set_target(&waypoint);

                // Simulate moving toward the waypoint
                while !self.reached_waypoint(&state.position, &waypoint, None) {
                    self.update(0.1, state, environment); 
            }
        }

        println!("Reached the goal!");
    }

    fn reached_waypoint(&self, position: &Position, waypoint: &Position, tolerance: Option<f32>) -> bool {
        let distance = position.norm(waypoint);
        distance < tolerance.unwrap_or(0.1)
    }
}