extern crate pt;

use std::rc::Rc;
use pt::*;
use math::*;
use color::XYZ;
use material::{Diffuse, Emission, Glass, Mirror};
use geometry::{Sphere, Disk};
use texture::Checkerboard;
use object::Object;
use renderer::Renderer;

fn main() {
    let mut scene = Scene::new(
        Camera::new(Matrix4::look_at(
            Point3::new(-2.0, -4.4, 3.4),
            Point3::new(-0.8, 0.2, 0.7),
            Vector3::unit_z()
        ).invert().unwrap(), 600, 400).fov(60.0).dof(5.0, 0.08)
    );

    scene.add(Object::new(
        Rc::new(Sphere::new(1.0)),
        Rc::new(Glass::new(1.50))
    ).transform(Matrix4::from_translation(Vector3::unit_z() * 1.0)));

    scene.add(Object::new(
        Rc::new(Sphere::new(0.7)),
        Rc::new(Mirror::new())
    ).transform(Matrix4::from_translation(Vector3::new(-2.4, 1.1, 0.7))));

    scene.add(Object::new(
        Rc::new(Sphere::new(1.1)),
        Rc::new(Diffuse::new(Rc::new(XYZ::new(0.7, 0.07, 0.07)), Rc::new(0.0)))
    ).transform(Matrix4::from_translation(Vector3::new(-0.4, 2.3, 1.1))));

    scene.add(Object::new(
        Rc::new(Disk::new(40.0)),
        Rc::new(Diffuse::new(Rc::new(
            Checkerboard::new(Rc::new(0.6), Rc::new(0.23), 40.0)
        ), Rc::new(0.0)))
    ));

    scene.add(Object::new(
        Rc::new(Disk::new(2.2)),
        Rc::new(Emission::new(Rc::new(XYZ::new(1.0, 0.9, 0.86) * 6.0)))
    ).transform(Matrix4::from_angle_y(Deg(30.0)) * Matrix4::from_translation(Vector3::unit_z() * 4.2)));

    Renderer::new(scene, 600, 400, 200, 3, 12)
    .render_to_file("render.png").unwrap();
}
