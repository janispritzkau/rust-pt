use std::rc::Rc;
use texture::Texture;

pub trait Material {

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

}
