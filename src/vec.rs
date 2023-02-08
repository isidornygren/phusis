use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Vec2> for Vec2 {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Add<Vec2> for f32 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: other.x + self,
            y: other.y + self,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl AddAssign<f32> for Vec2 {
    fn add_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
        };
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, other: Self) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<&Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl Sub<Vec2> for f32 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: other.x - self,
            y: other.y - self,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
        };
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<f32> for &Vec2 {
    type Output = Vec2;

    fn mul(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: other.x * self,
            y: other.y * self,
        }
    }
}

impl Mul<&Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: other.x * self,
            y: other.y * self,
        }
    }
}

impl Div for Vec2 {
    type Output = Vec2;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Div<f32> for &Vec2 {
    type Output = Vec2;

    fn div(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;

    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: other.x / self,
            y: other.y / self,
        }
    }
}
