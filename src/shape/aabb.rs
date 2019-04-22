use nalgebra::Vector2;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl AABB {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        return AABB {
            min: Vector2::new(x, y),
            max: Vector2::new(x + width, y + height),
        };
    }
    pub fn get_rect(&self) -> (f32, f32, f32, f32) {
        return (
            self.min.x,
            self.min.y,
            self.max.x - self.min.x,
            self.max.y - self.min.y,
        );
    }
    pub fn get_width(&self) -> f32 {
        return self.max.x - self.min.x;
    }
    pub fn get_height(&self) -> f32 {
        return self.max.y - self.min.y;
    }
    pub fn get_x(&self) -> f32 {
        self.min.x
    }
    pub fn get_y(&self) -> f32 {
        self.min.y
    }
    pub fn get_vertical_mid(&self) -> f32 {
        self.min.y + self.get_height() / 2f32
    }
    pub fn get_horizontal_mid(&self) -> f32 {
        self.min.x + self.get_width() / 2f32
    }
    /**
     * is strictly within another AABB
     */
    pub fn is_within(&self, other: &AABB) -> bool {
        return self.min.x > other.min.x
            && self.min.y > other.min.y
            && self.max.x < other.max.x
            && self.max.y < other.max.y;
    }
}

impl Add<Vector2<f32>> for AABB {
    type Output = Self;
    fn add(self, rhs: Vector2<f32>) -> AABB {
        AABB {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}
