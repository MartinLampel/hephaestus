
use crate::environment::Environment;
use crate::robot::state::RobotState;


pub enum SensorMeasurement {
    Range {
        distance: f32,           // Distance measured by a range sensor (e.g., LIDAR)
        angle: f32,              // Angle of the measurement relative to the robot
    },
    Data(Vec<SensorMeasurement>)
}


pub trait Sensor {
    fn measure(&self, state: &RobotState, environment: &Environment) -> SensorMeasurement;
}
