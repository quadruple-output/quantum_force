use super::components::{Camera, Damping};
use crate::common::{components::Velocity, events::CameraControl};
use bevy::prelude::*;

pub fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn((Camera,))
        .with_bundle(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(Velocity(Vec3::unit_x() * 5.0))
        .with(Damping::half_live(0.5));
}

pub fn aim(mut query: Query<(&Camera, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.look_at(Vec3::default(), Vec3::unit_y())
    }
}

pub fn control_camera(
    events: Res<Events<CameraControl>>,
    mut event_reader: Local<EventReader<CameraControl>>,
    mut query: Query<(&Camera, &Transform, &mut Velocity)>,
) {
    for event in event_reader.iter(&events) {
        for (_cam, _transform, mut v) in query.iter_mut() {
            *v += event.change_velocity;
        }
    }
}

pub fn slow_down(time: Res<Time>, mut query: Query<(&Damping, &mut Velocity)>) {
    const UNNOTICEABLE_VELOCITY: f32 = 0.01;

    let dt = time.delta();
    for (&damping, mut v) in query.iter_mut() {
        if *v >= UNNOTICEABLE_VELOCITY {
            assert!(damping >= 0.0); // don't want to accelerate
            *v *= 1.0 - damping * dt;
        } else if *v != Velocity::default() {
            *v = Velocity::default();
        }
    }
}
