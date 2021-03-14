use crate::{ray::Ray, vec3::Vec3};

pub trait Geometry {
    fn get_normal_at(&self, point: Vec3) -> Vec3;
    fn get_intersection(&self, ray: &Ray) -> Option<f64>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Geometry for Sphere {
    fn get_normal_at(&self, point: Vec3) -> Vec3 {
        (self.center - point).normalize()
    }

    fn get_intersection(&self, ray: &Ray) -> Option<f64> {
        let a = Vec3::dot(ray.ray_direction, ray.ray_direction);
        let b = 2.0 * Vec3::dot(ray.ray_direction, ray.ray_origin - self.center);

        let point_distance = ray.ray_origin - self.center;
        let radius_sqrd = self.radius * self.radius;

        let c = Vec3::dot(point_distance, point_distance) - radius_sqrd;

        let delta = b * b - 4.0 * a * c;

        if delta < 0.0 {
            None
        } else {
            let (t0, t1) = ((-b + delta.sqrt()) / 2.0 * a, (-b - delta.sqrt()) / 2.0 * a);
            match (t0, t1) {
                (t0, t1) if t0 > 0.0 && t1 > 0.0 => Some(t0.min(t1)),
                (t0, t1) if t0 < 0.0 && t1 > 0.0 || t0 > 0.0 && t1 < 0.0 => None,
                (t0, t1) if t0 == t1 => Some(t0),
                (_, _) => None,
            }
        }
    }
}
