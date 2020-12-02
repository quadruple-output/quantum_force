#[derive(Debug)]
pub struct Position(f32, f32, f32);

impl Position {
    pub fn origin() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn at(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }
}
