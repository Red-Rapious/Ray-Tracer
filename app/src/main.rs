use lib_ray_tracer::{camera::{Camera, self}, geometry::Sphere, world::World, Renderer, material::Material::{Lambertian, Metal}};
use nalgebra::{Point3, Vector3};

fn main() {
    let aspect_ratio = 16.0 / 9.0; // TODO: compute actual_ratio
    let image_width = 400;

    let camera = Camera::new(
        1.0,
        2.0,
        aspect_ratio,
        Point3::from([0.0, 0.0, 0.0]),
        100,
        10,
        camera::Gamma::Gamma2
    );

    let ground_mat = Lambertian(Vector3::new(0.8, 0.8, 0.0));
    let center_mat = Lambertian(Vector3::new(0.7, 0.3, 0.3));
    let left_mat = Metal(Vector3::new(0.8, 0.8, 0.8), 0.3);
    let right_mat = Metal(Vector3::new(0.8, 0.6, 0.2), 1.0);

    let mut world = World::empty();
    world.add(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, ground_mat));
    world.add(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, center_mat));
    world.add(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, left_mat));
    world.add(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, right_mat));

    let renderer = Renderer::new(aspect_ratio, image_width, camera);
    let img = renderer.render_image(&world);

    img.save("generated_images/13_fuzzed_metal.png")
        .unwrap();
}
