use crate::components::Position;

pub struct Sphere;

impl Sphere {
    pub fn bundle(pos: Position) -> (Self, Position) {
        (Self, pos)
    }
}
