use std::rc::Rc;
use rand::{XorShiftRng, Rng};
use texture::Texture;
use ray::*;
use math::*;
use std::mem;
use color::XYZ;
use std::f32::consts::PI;

enum MaterialSample {

}

pub trait Material {
    fn bsdf(&self, hit: &Intersection, ray: &Ray, rng: &mut XorShiftRng) -> (Ray, XYZ);
    fn emission(&self, hit: &Intersection) -> Option<XYZ>;
}

pub struct Diffuse {
    color: Rc<Texture>,
    roughness: Rc<Texture>
}

impl Diffuse {
    pub const fn new(color: Rc<Texture>, roughness: Rc<Texture>) -> Self {
        Self { color, roughness }
    }
}

impl Material for Diffuse {
    fn bsdf(&self, hit: &Intersection, _ray: &Ray, rng: &mut XorShiftRng) -> (Ray, XYZ) {
        let color = self.color.sample(hit.u, hit.v);
        (Ray {
            origin: hit.position + hit.normal * 0.0001,
            direction: random_hemisphere_direction(hit.normal, rng)
        }, color)
    }
    fn emission(&self, _: &Intersection) -> Option<XYZ> {
        None
    }
}

fn random_hemisphere_direction(normal: Vector3<f32>, rng: &mut XorShiftRng) -> Vector3<f32> {
    let mut u = normal.cross(Vector3::unit_x());
    if u.magnitude2() == 0.0 { u = normal.cross(Vector3::unit_y()) }
    let v = normal.cross(u);
    let phi = rng.gen::<f32>() * PI * 2.0;
    let theta = rng.gen::<f32>().asin();
    u * phi.cos() * theta.cos() + v * phi.sin() * theta.cos() + normal * theta.sin()
}

pub struct Emission {
    color: Rc<Texture>
}

impl Emission {
    pub fn new(color: Rc<Texture>) -> Self {
        Self { color }
    }
}

impl Material for Emission {
    fn bsdf(&self, _: &Intersection, _: &Ray, _: &mut XorShiftRng) -> (Ray, XYZ) {
        unimplemented!()
    }
    fn emission(&self, hit: &Intersection) -> Option<XYZ> {
        Some(self.color.sample(hit.u, hit.v))
    }
}

pub struct Glass {
    ior: f32
}

impl Glass {
    pub fn new(ior: f32) -> Self {
        Self { ior }
    }
}

impl Material for Glass {
    fn bsdf(&self, hit: &Intersection, ray: &Ray, rng: &mut XorShiftRng) -> (Ray, XYZ) {
        let mut etai = 1.0 / self.ior;
        let mut etat = 1.0;
        let n = hit.normal * if ray.direction.dot(hit.normal) > 0.0 {
            -1.0
        } else {
            mem::swap(&mut etai, &mut etat);
            1.0
        };

        let refl = reflect(n, ray.direction);
        let refr = refract(n, ray.direction, etat / etai);

        let r1 = refl.dot(n);
        let r2 = refr.dot(-n);
        let f = 1.0 - (
            ((etat * r1 - etai * r2) / (etat * r1 + etai * r2)).powi(2) +
            ((etai * r2 - etat * r1) / (etai * r2 + etat * r1)).powi(2)
        ) / 2.0;

        let ray = if f < rng.gen::<f32>() { Ray {
            origin: hit.position + n * 0.0001,
            direction: refl
        } } else { Ray {
            origin: hit.position - n * 0.0001,
            direction: refr
        } };
        (ray, XYZ::white())
    }

    fn emission(&self, _: &Intersection) -> Option<XYZ> {
        None
    }
}

fn refract(n: Vector3<f32>, i: Vector3<f32>, eta: f32) -> Vector3<f32> {
    let cos_i = -i.dot(n);
    let cos_t2 = 1.0 - eta.powi(2) * (1.0 - cos_i.powi(2));
    let t = i * eta + n * (eta * cos_i - cos_t2.abs().sqrt());
    t * if cos_t2 > 0.0 { 1.0 } else { 0.0 }
}

fn reflect(n: Vector3<f32>, i: Vector3<f32>) -> Vector3<f32> {
    i - n * 2.0 * n.dot(i)
}

pub struct Mirror {}

impl Mirror {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Mirror {
    fn bsdf(&self, hit: &Intersection, ray: &Ray, _: &mut XorShiftRng) -> (Ray, XYZ) {
        let ray = Ray {
            origin: hit.position + hit.normal * 0.0001,
            direction: reflect(hit.normal, ray.direction)
        };
        (ray, XYZ::white())
    }

    fn emission(&self, _: &Intersection) -> Option<XYZ> {
        None
    }
}
