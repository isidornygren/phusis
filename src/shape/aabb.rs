use crate::Vec2;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
    pub half: Vec2,
}

impl AABB {
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        AABB {
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height),
            half: Vec2::new(width / 2f32, height / 2f32),
        }
    }
    #[must_use]
    pub fn get_rect(&self) -> (f32, f32, f32, f32) {
        (
            self.min.x,
            self.min.y,
            self.max.x - self.min.x,
            self.max.y - self.min.y,
        )
    }
    #[must_use]
    pub fn get_width(&self) -> f32 {
        self.max.x - self.min.x
    }
    #[must_use]
    pub fn get_height(&self) -> f32 {
        self.max.y - self.min.y
    }
    #[must_use]
    pub fn get_x(&self) -> f32 {
        self.min.x
    }
    #[must_use]
    pub fn get_y(&self) -> f32 {
        self.min.y
    }
    #[must_use]
    pub fn get_vertical_mid(&self) -> f32 {
        self.min.y + self.half.y
    }
    #[must_use]
    pub fn get_horizontal_mid(&self) -> f32 {
        self.min.x + self.half.x
    }
    /**
     * is strictly within another AABB
     */
    #[must_use]
    pub fn is_within(&self, other: &AABB) -> bool {
        self.min.x > other.min.x
            && self.min.y > other.min.y
            && self.max.x < other.max.x
            && self.max.y < other.max.y
    }
}

impl Add<&Vec2> for &AABB {
    type Output = AABB;
    fn add(self, other: &Vec2) -> AABB {
        AABB {
            min: self.min.clone() + other,
            max: self.max.clone() + other,
            half: self.half.clone(),
        }
    }
}
