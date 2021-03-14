use crate::{geometry::Geometry, material::Material};

pub struct Surface<'a> {
    pub geometry: &'a dyn Geometry,
    pub material: Material,
}

impl<'a> Surface<'a> {
    pub fn new(geometry: &'a dyn Geometry, material: Material) -> Self {
        Surface { geometry, material }
    }
}
