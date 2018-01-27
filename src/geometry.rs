use math::*;
use ray::*;
use std::f32::consts::PI;
use std::f32::EPSILON;

pub trait Geometry {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub struct Sphere { radius: f32, radius_sqr: f32 }
impl Sphere { pub fn new(radius: f32) -> Self { Self { radius, radius_sqr: radius.powi(2) } } }
impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let l = Point3::origin() - ray.origin; let l_sqr = l.dot(l);
        let t_ca = l.dot(ray.direction); if t_ca < 0.0 && l_sqr > self.radius_sqr { return None }
        let d_sqr = l_sqr - t_ca.powi(2); if d_sqr > self.radius_sqr { return None }
        let t_hc = (self.radius_sqr - d_sqr).sqrt();
        let distance = if l_sqr > self.radius_sqr { t_ca - t_hc } else { t_ca + t_hc };
        let position = ray.origin + ray.direction * distance;
        Some(Intersection {
            distance, position, normal: position.to_vec() / self.radius,
            u: position.x.atan2(position.y) / (2.0 * PI),
            v: position.z / PI
        })
    }
}

fn intersect_plane(center: Point3<f32>, normal: Vector3<f32>, ray: &Ray) -> Option<f32> {
    let denom = normal.dot(ray.direction); if denom.abs() < EPSILON { return None }
    let t = (center - ray.origin).dot(normal) / denom; if t <= 0.0 { return None };
    Some(t)
}

pub struct Plane { scale: Vector2<f32>, normal: Vector3<f32> }
impl Plane { pub fn new(scale: Vector2<f32>) -> Self { Self { scale, normal: Vector3::unit_z() } } }
impl Geometry for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(distance) = intersect_plane(Point3::origin(), self.normal, ray) {
            let position = ray.origin + ray.direction * distance;
            if position.x.abs() > self.scale.x || position.y.abs() > self.scale.y { return None }
            Some(Intersection {
                distance, position, normal: self.normal,
                u: 0.5 + position.x / self.scale.x * 1.0,
                v: 0.5 + position.y / self.scale.y * 1.0
            })
        } else { None }
    }
}

pub struct Disk { radius: f32, radius_sqr: f32, normal: Vector3<f32> }
impl Disk { pub fn new(radius: f32) -> Self {
    Self { radius, radius_sqr: radius.powi(2), normal: Vector3::unit_z() }
} }
impl Geometry for Disk {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(distance) = intersect_plane(Point3::origin(), self.normal, ray) {
            let position = ray.origin + ray.direction * distance;
            if position.distance2(Point3::origin()) > self.radius_sqr { return None }
            Some(
                Intersection {distance, position, normal: self.normal,
                u: 0.5 + position.x / self.radius * 0.5,
                v: 0.5 + position.y / self.radius * 0.5
            })
        } else { None }
    }
}
