use crate::common::{components::Velocity, events::CameraControl, resources::PausePhysics};
use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Default)]
pub struct EventReaders {
    mouse_motion: EventReader<MouseMotion>,
    //mouse_wheel: EventReader<MouseWheel>,
}

pub fn react_on_input(
    mut event_readers: Local<EventReaders>,
    keyboard: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut out_events: ResMut<Events<CameraControl>>,
) {
    const MOUSE_SPEED: f32 = 0.02;

    for motion in event_readers.mouse_motion.iter(&mouse_motion_events) {
        if keyboard.pressed(KeyCode::LControl) || mouse_buttons.pressed(MouseButton::Right) {
            out_events.send(CameraControl {
                add_velocity: Velocity::new(0.0, -motion.delta.y * MOUSE_SPEED, 0.0),
            });
        }
    }
}

pub fn tmp_start_physics(
    time: Res<Time>,
    mut pause_physics: ResMut<PausePhysics>,
    mut started_once: Local<bool>,
) {
    if !*started_once && time.seconds_since_startup() > 3.0 {
        pause_physics.0 = false;
        *started_once = true;
    }
}
