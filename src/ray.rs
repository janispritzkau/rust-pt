use math::*;

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>
}

pub struct Intersection {
    pub distance: f32,
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
    pub u: f32,
    pub v: f32
}
