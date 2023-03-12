use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    #[inline]
    #[must_use]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vec2<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    #[inline]
    #[must_use]
    pub fn dot(&self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl<T> Vec2<T>
where
    T: Add<Output = T> + Copy,
{
    #[inline]
    #[must_use]
    pub fn len(&self) -> T {
        self.x + self.y
    }
}

impl Vec2<f32> {
    #[must_use]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Vec2<i32> {
    #[must_use]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Vec2<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<T> AddAssign<T> for Vec2<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: T) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
        };
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Sub<T> for Vec2<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn sub(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl<T> SubAssign for Vec2<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl<T> SubAssign<T> for Vec2<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: T) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
        };
    }
}

impl<T> Mul for Vec2<T>
where
    T: Mul<Output = T>,
{
    type Output = Vec2<T>;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> Div for Vec2<T>
where
    T: Div<Output = T>,
{
    type Output = Vec2<T>;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
