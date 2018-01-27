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
