use crate::vec3::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub plane_center: Vec3,
    pub plane_direction_x: Vec3,
    pub plane_direction_y: Vec3,
    pub width: usize,
    pub height: usize,
}

impl Camera {
    pub fn new(position: Vec3, direction: Vec3, width: usize, height: usize) -> Self {
        let vector_up = Vec3::new(0.0, -1.0, 0.0);
        let plane_center = Vec3::add(position, 430.0 * direction);
        let plane_direction_x = Vec3::cross(direction, vector_up).normalize();
        let plane_direction_y = Vec3::cross(direction * -1.0, plane_direction_x);

        Camera {
            position,
            direction,
            width,
            height,
            plane_center,
            plane_direction_x,
            plane_direction_y,
        }
    }
}
