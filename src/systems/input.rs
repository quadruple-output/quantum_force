use crate::Velocity;
use crate::{MainCamera, PausePhysics};
use bevy::{
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseMotion, MouseWheel},
    },
    prelude::*,
};

#[derive(Default)]
pub struct EventReaders {
    mouse_motion: EventReader<MouseMotion>,
    mouse_wheel: EventReader<MouseWheel>,
}

pub fn react_on_input(
    pause: Res<PausePhysics>,
    mut event_readers: Local<EventReaders>,
    keyboard: Res<Input<KeyCode>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut camera_query: Query<(&MainCamera, &mut Velocity)>,
) {
    const MOUSE_SPEED: f32 = 0.02;

    if pause.0 {
        // consume pending events:
        for _ in event_readers.mouse_motion.iter(&mouse_motion_events) {}
        return;
    };
    for motion in event_readers.mouse_motion.iter(&mouse_motion_events) {
        if keyboard.pressed(KeyCode::LControl) {
            for (_camera, mut v) in camera_query.iter_mut() {
                v.0.y -= motion.delta.y * MOUSE_SPEED;
            }
        }
    }
}
