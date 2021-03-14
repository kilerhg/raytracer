#[allow(dead_code)]
use crate::{camera::Camera, light::Light, surface::Surface, vec3::Vec3};

pub struct Scene<'a> {
    pub surfaces: Vec<&'a Surface<'a>>,
    pub lights: Vec<&'a Light>,
    pub ambient_coefficient: f64,
    pub camera: &'a Camera,
}

impl<'a> Scene<'a> {
    pub fn new(camera: &'a Camera) -> Self {
        Scene {
            surfaces: Vec::new(),
            lights: Vec::new(),
            ambient_coefficient: 0.6,
            camera,
        }
    }

    pub fn add_light(&mut self, light: &'a Light) {
        self.lights.push(light);
    }

    pub fn add_lights(&mut self, lights: Vec<&'a Light>) {
        for light in lights.iter() {
            self.lights.push(light);
        }
    }

    pub fn add_surface(&mut self, surface: &'a Surface) {
        self.surfaces.push(surface);
    }

    pub fn add_surfaces(&mut self, surfaces: Vec<&'a Surface>) {
        for surface in surfaces.iter() {
            self.surfaces.push(surface);
        }
    }
}
