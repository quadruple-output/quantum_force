use bevy::prelude::*;
pub struct PausePhysics(pub bool);

impl FromResources for PausePhysics {
    fn from_resources(_resources: &Resources) -> Self {
        Self(true)
    }
}
