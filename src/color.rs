use std::ops::{Add, Sub, Div, Mul};

use std::path::Path;

#[derive(Copy, Clone)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl XYZ {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn white() -> Self { Self::new(1.0, 1.0, 1.0) }
    pub fn black() -> Self { Self::new(0.0, 0.0, 0.0) }

    pub fn luminance(&self) -> f32 {
        self.x * 0.2126 + self.y * 0.7152 + self.z * 0.0722
    }

    pub fn to_srgb(self) -> Self {
        let c = |c: f32| c.powf(1.0 / 2.2).max(0.0).min(1.0);
        Self::new(c(self.x), c(self.y), c(self.z))
    }
}

impl Add for XYZ { type Output = Self; fn add(self, other: Self) -> Self {
    Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
} }
impl Sub for XYZ { type Output = Self; fn sub(self, other: Self) -> Self {
    Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
} }
impl Mul for XYZ { type Output = Self; fn mul(self, other: Self) -> Self {
    Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
} }
impl Mul<f32> for XYZ { type Output = Self; fn mul(self, other: f32) -> Self {
    Self::new(self.x * other, self.y * other, self.z * other)
} }
impl Div<f32> for XYZ { type Output = Self; fn div(self, other: f32) -> Self {
    Self::new(self.x / other, self.y / other, self.z / other)
} }
