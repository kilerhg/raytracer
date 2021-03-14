use crate::color::Color;

pub struct Material {
    color: Color,
    reflectivity: f64,
    reflection_noise: f64,
    specularity: f64,
}

impl Material {
    pub fn new(color: Color, reflectivity: f64, reflection_noise: f64, specularity: f64) -> Self {
        Material {
            color,
            reflectivity,
            reflection_noise,
            specularity,
        }
    }
}
