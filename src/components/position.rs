#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn at(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
