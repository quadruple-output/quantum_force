#[derive(Debug)]
pub struct Repulsor {
    /// logarithmic value. <0.1 may cause particles to tunnel or shoot back
    pub range: f32,
}
