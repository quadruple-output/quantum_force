use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct MainCamera;

impl MainCamera {
    pub fn aim(mut query: Query<(&MainCamera, &mut Transform)>) {
        for (_, mut transform) in query.iter_mut() {
            transform.look_at(Vec3::default(), Vec3::unit_y())
        }
    }
}
