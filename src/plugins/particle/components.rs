use bevy::ecs::Entity;

pub struct LeadingParticle(pub Entity);

#[derive(Copy, Clone)]
pub enum Spin {
    Up,
    Down,
}
