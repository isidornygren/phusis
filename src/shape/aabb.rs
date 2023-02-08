use crate::shape::{Shape, ShapeKind};
use crate::Vec2;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
    pub half: Vec2,
}

impl AABB {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        return AABB {
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height),
            half: Vec2::new(width / 2f32, height / 2f32),
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
        self.min.y + self.half.y
    }
    pub fn get_horizontal_mid(&self) -> f32 {
        self.min.x + self.half.x
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

impl Add<&Vec2> for AABB {
    type Output = Self;
    fn add(self, other: &Vec2) -> AABB {
        AABB {
            min: self.min + other,
            max: self.max + other,
            half: self.half,
        }
    }
}

impl Shape for AABB {
    fn get_kind(&self) -> ShapeKind {
        return ShapeKind::AABB;
    }
    fn get_radius(&self) -> f32 {
        let height = self.get_height();
        let width = self.get_width();
        if width > height {
            return width / 2f32;
        } else {
            return height / 2f32;
        }
    }
    fn get_aabb(&self) -> AABB {
        return self.clone();
    }
}
