#![feature(const_fn)]

extern crate cgmath;
extern crate image;
extern crate rand;
extern crate rand_xorshift;

pub mod math {
    pub use cgmath::{Vector2, Vector3, Point3, Matrix3, Matrix4, Deg,
        Transform, EuclideanSpace, ElementWise, InnerSpace, MetricSpace, SquareMatrix};
}

pub mod ray;
pub mod camera;
pub mod scene;
pub mod geometry;
pub mod object;
pub mod material;
pub mod color;
pub mod texture;
pub mod renderer;

pub use camera::Camera;
pub use scene::Scene;
