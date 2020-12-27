use super::components::{Camera, Damping};
use crate::common::components::CameraControl;
use angle::Rad;
use bevy::prelude::*;

pub fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn((Camera,))
        .with_bundle(Camera3dBundle {
            transform:
                //Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                //.looking_at(Vec3::default(), Vec3::unit_y()),
                Transform::from_translation(Vec3::new(0.0, 4.0, 8.0))
                .looking_at(Vec3::new(0.0,1.2,0.0), Vec3::unit_y()),
            ..Default::default()
        })
        .with(CameraControl {
            yaw: Rad(2.5),
            ..Default::default()
        })
        .with(Damping::half_live(0.5));
}

pub fn control_camera(
    events: Res<Events<CameraControl>>,
    mut event_reader: Local<EventReader<CameraControl>>,
    mut query: Query<(&Camera, &mut CameraControl)>,
) {
    for &event in event_reader.iter(&events) {
        for (_cam, mut cc) in query.iter_mut() {
            *cc += event;
        }
    }
}

pub fn slow_down(time: Res<Time>, mut query: Query<(&Damping, &mut CameraControl)>) {
    const UNNOTICEABLE_VELOCITY: f32 = 0.000001;

    let dt = time.delta();
    for (&damping, mut cc) in query.iter_mut() {
        if *cc >= UNNOTICEABLE_VELOCITY {
            assert!(damping >= 0.0); // don't want to accelerate
            *cc *= 1.0 - damping * dt;
        } else if *cc != CameraControl::default() {
            *cc = CameraControl::default();
        }
    }
}

pub fn apply_control(time: Res<Time>, mut query: Query<(&CameraControl, &mut Transform)>) {
    let look_at_target = Vec3::new(0.0, 1.2, 0.0); // TODO

    let dt = time.delta_seconds();
    for (&cc, mut transform) in query.iter_mut() {
        let ypr = Quat::from_rotation_ypr(cc.yaw.0 * dt, cc.pitch.0 * dt, 0.0);
        let new_rotation = (transform.rotation * ypr).normalize();
        let v_target_to_cam = transform.translation - look_at_target;
        let mut new_v_target_to_cam =
            new_rotation * transform.rotation.conjugate() * v_target_to_cam;
        new_v_target_to_cam += new_v_target_to_cam * (cc.escape_velocity.0 * dt);
        // don't go closer than one unit:
        if new_v_target_to_cam.length_squared() < 1.0 {
            new_v_target_to_cam = new_v_target_to_cam.normalize();
        }
        transform.rotation = new_rotation;
        transform.translation = look_at_target + new_v_target_to_cam;
    }
}
