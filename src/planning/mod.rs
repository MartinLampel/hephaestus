

use crate::environment::Environment;
use crate::robot::state::Position;

pub trait PathPlanner {
    fn plan_path(&self, start: &Position, goal: &Position, environment: &Environment) -> Vec<Position>;
}