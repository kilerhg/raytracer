use crate::vec3::Vec3;

pub struct Light {
    position: Vec3,
    intensity: f64,
}

impl Light {
    fn new(position: Vec3, intensity: f64) -> Self {
        Light {
            position,
            intensity,
        }
    }

    fn direction(&self, point: Vec3) -> Vec3 {
        (point - self.position).normalize()
    }

    fn difused_highlight(&self, light_direction: Vec3, normal: Vec3) -> f64 {
        let highlight = Vec3::dot(normal, light_direction);

        let difused_unlocked = highlight * self.intensity;
        difused_unlocked.clamp(0.0, std::f64::MAX)
    }

    fn specular_highlight(
        &self,
        light_direction: Vec3,
        normal: Vec3,
        ray_direction: Vec3,
        specularity: f64,
    ) -> f64 {
        let highlight = Vec3::dot(normal, light_direction);
        let v = -1.0 * ray_direction;
        let r = light_direction - (normal * highlight * 2.0);
        let dot = Vec3::dot(v, r);

        let specular_unlocked = dot.powf(specularity) * 1.0 * self.intensity;
        specular_unlocked.clamp(0.0, std::f64::MAX)
    }
}
