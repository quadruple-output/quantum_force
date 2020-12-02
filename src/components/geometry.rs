#[derive(Debug)]
pub enum BasicShape {
    #[allow(dead_code)]
    Sphere { radius: f32 },
}

impl BasicShape {
    pub fn sphere(radius: f32) -> Self {
        Self::Sphere { radius }
    }
}
