#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    #[must_use]
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}
