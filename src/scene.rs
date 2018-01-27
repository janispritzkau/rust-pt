use std;
use std::rc::Rc;
use camera::Camera;
use object::Object;
use material::Material;
use ray::*;

pub struct Scene {
    pub camera: Camera,
    objects: Vec<Object>
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self { camera, objects: Vec::new() }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(Intersection, Rc<Material>)> {
        let mut nearest = std::f32::INFINITY;
        let mut nearest_hit: Option<(Intersection, Rc<Material>)> = None;
        for object in &self.objects {
            if let Some((hit, material)) = object.intersect(ray) {
                if hit.distance < nearest {
                    nearest = hit.distance;
                    nearest_hit = Some((hit, material));
                }
            }
        }
        nearest_hit
    }
}
