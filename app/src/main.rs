use lib_ray_tracer::{camera::Camera, geometry::Sphere, world::World, Renderer};
use nalgebra::Point3;

fn main() {
    let aspect_ratio = 16.0 / 9.0; // TODO: compute actual_ratio
    let image_width = 400;

    let camera = Camera::new(1.0, 2.0, aspect_ratio, Point3::from([0.0, 0.0, 0.0]), 100);

    let mut world = World::empty();
    world.add(Sphere::new(Point3::from([0.0, 0.0, -1.0]), 0.5));
    world.add(Sphere::new(Point3::from([0.0, -100.5, -1.0]), 100.0));

    let renderer = Renderer::new(aspect_ratio, image_width, camera);
    let img = renderer.render_image(&world);

    img.save("generated_images/antialiasing_100.png").unwrap();
}
