extern crate pt;

use std::rc::Rc;
use pt::*;
use math::*;
use color::XYZ;
use material::Diffuse;
use geometry::{Sphere, Disk};
use object::Object;
use renderer::Renderer;

fn main() {
    let mut scene = Scene::new(
        Camera::new(Matrix4::look_at(
            Point3::new(-1.0, -4.0, 2.0),
            Point3::new(0.0, 0.0, 0.5),
            Vector3::unit_z()
        ).invert().unwrap(), 800, 600).fov(70.0).dof(3.0, 0.1)
    );

    scene.add(Object::new(
        Rc::new(Sphere::new(1.0)),
        Rc::new(Diffuse::new(Rc::new(XYZ::new(0.7, 0.7, 0.7)), Rc::new(0.0)))
    ));

    scene.add(Object::new(
        Rc::new(Disk::new(24.0)),
        Rc::new(Diffuse::new(Rc::new(XYZ::new(0.7, 0.7, 0.7)), Rc::new(0.0)))
    ));

    Renderer::new(scene, 800, 600, 16, 3, 12)
    .render_to_file("render.png").unwrap();
}
