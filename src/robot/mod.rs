

pub mod state;
pub mod sensors;
pub mod control;
pub mod kinematics;

pub use state::RobotState;
pub use sensors::Sensor;
pub use control::ControlAlgorithm;
pub use control::ControlData;
pub use kinematics::KinematicsModel;

use crate::planning::LocalPathPlanner;
use crate::prelude::*;

pub struct Robot {
    pub sensors: Vec<Box<dyn Sensor>>,
    pub control: Box<dyn ControlAlgorithm>,
    pub localization: Box<dyn LocalizationAlgorithm>,
    pub global_planner: Box<dyn GlobalPathPlanner>,
    pub local_planner: Box<dyn LocalPathPlanner>,
    pub kinematics: Box<dyn KinematicsModel>,
    dt: f32,
}

impl Robot {

    pub fn with_sensor(mut self, sensor: Box<dyn Sensor>) -> Self {
        self.sensors.push(sensor);
        self
    }

    pub fn localize(&mut self, state: &RobotState, environment: &Environment, dt: f32) {
        let mut measurements = Vec::new();
        for sensor in &self.sensors {
            measurements.push(sensor.measure(state, environment));
        }

        self.localization.update(state, &measurements, dt)
    }

    
    pub fn run_motion_control(&mut self, dt: f32, state: &mut RobotState, target_velocity: &ControlData) {
        
        let control_output = self.control.calculate_input(state, target_velocity,  dt);
        
        *state = self.kinematics.predict_state(state, &control_output, dt);

        state.odometry.update(control_output.velocity * dt, 
            0.0, 
            control_output.angular_velocity * dt);       

        state.velocity.linear = control_output.velocity;
        state.velocity.angular = control_output.angular_velocity;
    }

    pub fn plan_path_and_execute(&mut self, goal: Position, state: &mut RobotState, environment: &Environment) -> Vec<RobotState> {
        let path = self.global_planner.plan_path(&state.position, &goal, environment);
        let mut robot_states: Vec<RobotState> = Vec::new();

        for waypoint in &path {
                while !self.reached_waypoint(&state.position, &waypoint, None) {                  ;
                    let target_velocity = self.local_planner.plan_path(&state.position, waypoint, environment);
                    self.run_motion_control(self.dt, state, &target_velocity); 
                    self.localize(state, environment, self.dt);
                    robot_states.push(state.clone());
            }
        }

        println!("Reached the goal!");

        robot_states
    }

    fn reached_waypoint(&self, position: &Position, waypoint: &Position, tolerance: Option<f32>) -> bool {
        let distance = position.norm(waypoint);
        distance < tolerance.unwrap_or(0.1)
    }
}