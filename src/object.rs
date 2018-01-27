use std::rc::Rc;
use math::*;
use ray::*;
use geometry::Geometry;
use material::Material;

pub struct Object {
    geometry: Rc<Geometry>,
    material: Rc<Material>,
    transform: Matrix4<f32>, transform_inv: Matrix4<f32>
}

impl Object {
    pub fn new(geometry: Rc<Geometry>, material: Rc<Material>) -> Self {
        Self {
            geometry, material, transform: Matrix4::one(), transform_inv: Matrix4::one()
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(Intersection, Rc<Material>)> {
        if let Some(mut hit) = self.geometry.intersect(&Ray {
            origin: self.transform_inv.transform_point(ray.origin),
            direction: self.transform_inv.transform_vector(ray.direction)
        }) {
            hit.position = self.transform.transform_point(hit.position);
            hit.normal = self.transform.transform_vector(hit.normal);
            Some((hit, self.material.clone()))
        } else { None }
    }
}
