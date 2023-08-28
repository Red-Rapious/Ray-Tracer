use lib_ray_tracer::{
    camera::{self, Camera},
    geometry::Sphere,
    material::Material,
    world::World,
    Renderer,
};
use nalgebra::{Point3, Vector3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 640;

    let camera = Camera::new(
        1.0,
        Point3::from([0.0, 0.0, 0.0]),
        100,
        10,
        120.0,
        camera::Gamma::Gamma2,
    );

    let ground_mat = Material::Lambertian(Vector3::new(0.8, 0.8, 0.0));
    let center_mat = Material::Lambertian(Vector3::new(0.1, 0.2, 0.5));
    let left_mat = Material::Dielectric(1.5);
    let right_mat = Material::Metal(Vector3::new(0.8, 0.6, 0.2), 0.0);

    let mut world = World::empty();
    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        ground_mat,
    ));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, center_mat));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, left_mat));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, left_mat)); // hollow sphere effect
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, right_mat));

    let renderer = Renderer::new(aspect_ratio, image_width, camera);
    //let img = renderer.render_image(&world);
    let img = renderer.render_parallel_image(&world);

    img.save("generated_images/17_fov.png")
        .unwrap();
}
