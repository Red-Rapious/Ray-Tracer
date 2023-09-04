use lib_ray_tracer::{
    camera::{self, Camera},
    geometry::Sphere,
    material::Material,
    world::World,
    Renderer,
};
use nalgebra::{Point3, Vector3};
use rand::{thread_rng, Rng};

pub fn render() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 426;

    let camera = Camera::new(
        50,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        camera::Gamma::Gamma2,
        0.6,
        10.0,
    );

    let mut world = World::empty();
    let mut rng = thread_rng();

    let ground_mat = Material::Lambertian(Vector3::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let choose_mat: f64 = rng.gen();

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vector3::<f64>::new(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    );
                    let sphere_material = Material::Lambertian(albedo);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vector3::new(
                        rng.gen_range(0.5..=1.0),
                        rng.gen_range(0.5..=1.0),
                        rng.gen_range(0.5..=1.0),
                    );
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let sphere_material = Material::Metal(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Material::Dielectric(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Material::Dielectric(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertian(Vector3::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::Metal(Vector3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let renderer = Renderer::new(aspect_ratio, image_width, camera);
    let img = renderer.render_parallel_image(&world);

    img.save("generated_images/test.png").unwrap();
}
