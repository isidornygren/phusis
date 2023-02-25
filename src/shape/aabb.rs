use std::ops::{Add, Div, Sub};

use crate::Vec2;

#[derive(Debug, Clone)]
pub struct AABB<T> {
    pub min: Vec2<T>,
    pub max: Vec2<T>,
}

impl<T> AABB<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + PartialOrd + Copy,
{
    #[must_use]
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height),
        }
    }

    // #[must_use]
    // pub fn get_rect(&self) -> (T, T, T, T) {
    //     (self.min.x, self.min.y, self.width(), self.height())
    // }

    #[must_use]
    pub fn width(&self) -> T {
        self.max.x - self.min.x
    }

    #[must_use]
    pub fn height(&self) -> T {
        self.max.y - self.min.y
    }

    /**
     * is strictly within another AABB
     */
    #[must_use]
    pub fn is_within(&self, other: &AABB<T>) -> bool {
        self.min.x > other.min.x
            && self.min.y > other.min.y
            && self.max.x < other.max.x
            && self.max.y < other.max.y
    }
}
