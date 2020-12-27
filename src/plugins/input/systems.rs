use crate::common::types::RadPerSecond;
use crate::common::{components::CameraControl, resources::PausePhysics, types::PercentPerSecond};
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use core::f32::consts::TAU;

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
    const RAD_PER_SECOND_PER_MOUSE_PIXEL: f32 = 0.1 * TAU / 360.0; // degrees->radians

    let control_state = ControlState {
        keyboard: &keyboard,
        mouse_buttons: &mouse_buttons,
    };
    let mut camera_control = CameraControl::default();

    event_readers
        .mouse_motion
        .iter(&mouse_motion_events)
        .for_each(|motion| {
            if control_state.camera_movement_yaw_pitch() {
                camera_control.yaw +=
                    RadPerSecond(RAD_PER_SECOND_PER_MOUSE_PIXEL * -motion.delta.x);
                camera_control.pitch +=
                    RadPerSecond(RAD_PER_SECOND_PER_MOUSE_PIXEL * -motion.delta.y);
            } else if control_state.camera_movement_roll_pitch() {
                camera_control.roll +=
                    RadPerSecond(RAD_PER_SECOND_PER_MOUSE_PIXEL * motion.delta.x);
                camera_control.pitch +=
                    RadPerSecond(RAD_PER_SECOND_PER_MOUSE_PIXEL * -motion.delta.y);
            };
        });
    event_readers
        .mouse_wheel
        .iter(&mouse_wheel_events)
        .for_each(|scroll| {
            if control_state.camera_movement_yaw_pitch() {
                camera_control.escape_velocity += ESCAPE_VELOCITY_PER_WHEEL_UNIT * scroll.y;
            }
        });
    if !camera_control.is_initial() {
        out_events.send(camera_control);
    }
}

struct ControlState<'a> {
    keyboard: &'a Res<'a, Input<KeyCode>>,
    mouse_buttons: &'a Res<'a, Input<MouseButton>>,
}

impl<'a> ControlState<'a> {
    fn camera_movement_yaw_pitch(&self) -> bool {
        self.keyboard.pressed(KeyCode::LControl) || self.mouse_buttons.pressed(MouseButton::Right)
    }

    fn camera_movement_roll_pitch(&self) -> bool {
        self.keyboard.pressed(KeyCode::LAlt) || self.mouse_buttons.pressed(MouseButton::Left)
    }
}
