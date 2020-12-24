use bevy::prelude::*;

/// calculates the Transformation appropriate for the "shadow" cast by a Particle
pub fn shadow_transform(particle_translation: Vec3) -> Transform {
    let xz_position = Vec3::new(particle_translation.x, 0.01, particle_translation.z);
    Transform::from_translation(xz_position) * Transform::from_scale(Vec3::new(0.2, 0.0, 0.2))
}
