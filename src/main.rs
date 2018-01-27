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
            Point3::new(-2.0, -4.3, 3.2),
            Point3::new(-0.3, 0.0, 0.5),
            Vector3::unit_z()
        ).invert().unwrap(), 800, 600).fov(60.0).dof(4.3, 0.06)
    );

    scene.add(Object::new(
        Rc::new(Sphere::new(1.0)),
        Rc::new(Diffuse::new(Rc::new(XYZ::new(0.7, 0.7, 0.7)), Rc::new(0.0)))
    ).transform(Matrix4::from_translation(Vector3::unit_z() * 1.0)));

    scene.add(Object::new(
        Rc::new(Disk::new(32.0)),
        Rc::new(Diffuse::new(Rc::new(XYZ::new(0.7, 0.7, 0.7)), Rc::new(0.0)))
    ));

    Renderer::new(scene, 800, 600, 16, 3, 12)
    .render_to_file("render.png").unwrap();
}
