use std::rc::Rc;
use color::XYZ;

pub trait Texture {
    fn sample(&self, u: f32, v: f32) -> XYZ;
    fn sample_value(&self, u: f32, v: f32) -> f32;
}

impl Texture for XYZ {
    fn sample(&self, _: f32, _: f32) -> XYZ { *self }
    fn sample_value(&self, _: f32, _: f32) -> f32 { self.luminance() }
}

impl Texture for f32 {
    fn sample(&self, _: f32, _: f32) -> XYZ { XYZ::new(*self, *self, *self) }
    fn sample_value(&self, _: f32, _: f32) -> f32 { *self }
}

pub struct Checkerboard {
    color_a: Rc<Texture>,
    color_b: Rc<Texture>,
    scale: f32
}

impl Checkerboard {
    pub fn new(color_a: Rc<Texture>, color_b: Rc<Texture>, scale: f32) -> Self {
        Self { color_a, color_b, scale }
    }
}

impl Texture for Checkerboard {
    fn sample(&self, u: f32, v: f32) -> XYZ {
        if ((u * self.scale).floor() + (v * self.scale).floor()) % 2.0 < 1.0 {
            self.color_a.sample(u, v)
        } else {
            self.color_b.sample(u, v)
        }
    }

    fn sample_value(&self, u: f32, v: f32) -> f32 {
        self.sample(u, v).luminance()
    }
}
