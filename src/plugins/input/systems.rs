use crate::common::{components::CameraControl, resources::PausePhysics, types::PercentPerSecond};
use angle::{Deg, Rad};
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Default)]
pub struct EventReaders {
    mouse_motion: EventReader<MouseMotion>,
    mouse_wheel: EventReader<MouseWheel>,
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

pub fn react_on_input(
    mut event_readers: Local<EventReaders>,
    keyboard: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut out_events: ResMut<Events<CameraControl>>,
) {
    const ESCAPE_VELOCITY_PER_WHEEL_UNIT: PercentPerSecond<f32> = PercentPerSecond(0.4);
    const ANGLES_PER_MOUSE_PIXEL: Deg<f32> = Deg(0.1);

    let control_state = ControlState {
        keyboard: &keyboard,
        mouse_buttons: &mouse_buttons,
    };
    let mut camera_control = CameraControl::default();

    for motion in event_readers.mouse_motion.iter(&mouse_motion_events) {
        if control_state.camera_movement_active() {
            camera_control.yaw += Rad::from(ANGLES_PER_MOUSE_PIXEL * -motion.delta.x);
            camera_control.pitch += Rad::from(ANGLES_PER_MOUSE_PIXEL * -motion.delta.y);
        }
    }
    for scroll in event_readers.mouse_wheel.iter(&mouse_wheel_events) {
        if control_state.camera_movement_active() {
            camera_control.escape_velocity += ESCAPE_VELOCITY_PER_WHEEL_UNIT * scroll.y;
        }
    }
    if camera_control != CameraControl::default() {
        out_events.send(camera_control);
    }
}

struct ControlState<'a> {
    keyboard: &'a Res<'a, Input<KeyCode>>,
    mouse_buttons: &'a Res<'a, Input<MouseButton>>,
}

impl<'a> ControlState<'a> {
    fn camera_movement_active(&self) -> bool {
        self.keyboard.pressed(KeyCode::LControl) || self.mouse_buttons.pressed(MouseButton::Right)
    }
}
