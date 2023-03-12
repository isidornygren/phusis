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
    #[inline]
    #[must_use]
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height),
        }
    }

    #[inline]
    #[must_use]
    pub fn width(&self) -> T {
        self.max.x - self.min.x
    }

    #[inline]
    #[must_use]
    pub fn height(&self) -> T {
        self.max.y - self.min.y
    }

    #[inline]
    #[must_use]
    pub fn intersects(&self, other: &AABB<T>) -> bool {
        !(other.min.x > self.max.x
            || other.max.x < self.min.x
            || other.min.y < self.max.y
            || other.min.y > self.max.y)
    }
}
