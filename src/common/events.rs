use crate::plugins::camera::Plugin;

use super::components::Velocity;

pub struct CameraControl {
    pub add_velocity: Velocity<Plugin>,
}
