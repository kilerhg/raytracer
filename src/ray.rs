use crate::{camera::Camera, color::Color, scene::Scene, surface::Surface, vec3::Vec3};

pub struct Ray {
    pub ray_direction: Vec3,
    pub ray_origin: Vec3,
}

impl Ray {
    pub fn new(ray_direction: Vec3, ray_origin: Vec3) -> Self {
        Ray {
            ray_direction,
            ray_origin,
        }
    }

    pub fn from_pixel(camera: &Camera, x: usize, y: usize) -> Self {
        let delta_y = 1.0;
        let delta_x = 1.0;

        let pixel_y = (camera.height as f64 * -1.0 / 2.0) + delta_y * (y as f64 + 0.5);
        let pixel_x = (camera.width as f64 * -1.0 / 2.0) + delta_x * (x as f64 + 0.5);

        let vec_x = pixel_x * camera.plane_direction_x;
        let vec_y = pixel_y * camera.plane_direction_y;
        let pixel_vector = camera.plane_center + vec_x + vec_y;

        let ray_direction = (pixel_vector - camera.position).normalize();

        Ray::new(ray_direction, camera.position)
    }

    pub fn trace_once(&self, scene: &Scene, depth: u32) -> Color {
        let mut color = Color::from_hex(0x00);

        for object in scene.surfaces.iter() {
            if let Some(_) = object.geometry.get_intersection(self) {
                color = Color::from_hex(0xFF);
                break;
            }
        }

        color
    }

    // pub fn reflect(&self, surface: &Surface, point: Vec3) {}
    // pub fn intersect(&self, surface: &Surface, distance: f64) {}
}
