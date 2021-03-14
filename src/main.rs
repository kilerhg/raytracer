#![allow(dead_code)]
mod camera;
mod color;
mod geometry;
mod light;
mod material;
mod ray;
mod scene;
mod surface;
mod vec3;

use camera::Camera;
use color::Color;
use geometry::Sphere;
use material::Material;
use ray::Ray;
use scene::Scene;
use surface::Surface;
use vec3::Vec3;

fn main() {
    const WINDOW_WIDTH: usize = 800;
    const WINDOW_HEIGHT: usize = 600;

    let camera = Camera::new(Vec3::zero(), Vec3::zero(), WINDOW_WIDTH, WINDOW_HEIGHT);

    const BUFFER_SIZE: usize = WINDOW_WIDTH * WINDOW_HEIGHT;

    let mut render_buffer: Vec<Color> = Vec::with_capacity(BUFFER_SIZE);

    let mut scene = Scene::new(&camera);

    let color_blue = Color::from_hex(0x338DFF);

    let default_material = Material::new(color_blue, 0.5, 0.0, 0.5);
    let sphere = Sphere::new(Vec3::new(300.0, 200.0, 0.0), 100.0);

    let default_surface = Surface::new(&sphere, default_material);

    scene.add_surface(&default_surface);

    for x in 0..WINDOW_WIDTH - 1 {
        for y in 0..WINDOW_HEIGHT - 1 {
            let ray = Ray::from_pixel(&camera, x, y);
            let color = ray.trace_once(&scene, 1);

            println!("{}", camera.plane_direction_y);
            render_buffer.push(color);
        }
    }
}

#[test]
fn it_adds() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(1.0, 2.0, 3.0);

    let c = Vec3::new(2.0, 4.0, 6.0);

    let sum = a + b;

    assert_eq!(c.x, sum.x);
    assert_eq!(c.y, sum.y);
    assert_eq!(c.z, sum.z);
}
